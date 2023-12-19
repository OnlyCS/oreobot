use crate::prelude::*;

pub async fn create(
    member: serenity::Member,
    bot: &mut Client<BotServer>,
) -> Result<(), MemberLogError> {
    let prisma = prisma::create().await?;

    let color_role = {
        let proles = prisma
            .role()
            .find_many(vec![])
            .exec()
            .await?
            .into_iter()
            .filter(|r| r.color_role)
            .fold(HashMap::new(), |mut collect, item| {
                collect.insert(serenity::RoleId::new(item.id as u64), item);
                collect
            });

        let roles = member
            .roles
            .into_iter()
            .filter(|item| proles.get(&item).is_some())
            .collect_vec();

        if roles.is_empty() {
            let BotResponse::CreateRoleOk(role) = bot
                .send(BotRequest::CreateColorRole(member.user.id))
                .await?
            else {
                bail!(RouterError::InvalidResponse)
            };

            role.id
        } else {
            roles[0]
        }
    };

    let mut roles = vec![color_role];

    if member.user.bot {
        roles.push(nci::roles::BOTS);
    }

    if let Some(user) = prisma
        .user()
        .find_unique(user::id::equals(member.user.id))
        .exec()
        .await?
    {
        if user.admin {
            roles.push(nci::roles::OVERRIDES);
        }

        if user.verified {
            roles.push(nci::roles::MEMBERS);
        }

        prisma
            .user()
            .update(
                user::id::equals(user.id.clone()),
                vec![
                    user::username::set(member.user.name),
                    user::nickname::set(member.nick),
                    user::removed::set(false),
                    user::bot::set(member.user.bot),
                    user::roles::set(roles.iter().map(|n| role::id::equals(*n)).collect_vec()),
                ],
            )
            .exec()
            .await?;

        for role in roles {
            bot.send(BotRequest::AddRoleToUser(member.user.id, role))
                .await?;
        }
    } else {
        prisma
            .user()
            .create(
                member.user.id,
                member.user.name,
                vec![
                    user::nickname::set(member.nick),
                    user::roles::connect(roles.iter().map(|n| role::id::equals(*n)).collect_vec()),
                    user::bot::set(member.user.bot),
                ],
            )
            .exec()
            .await?;
    }

    Ok(())
}

pub async fn update(
    member: serenity::GuildMemberUpdateEvent,
    bot: &mut Client<BotServer>,
) -> Result<(), MemberLogError> {
    let prisma = prisma::create().await?;
    let mut updates = vec![];

    let roles = {
        let BotResponse::RolesOk(roles) =
            bot.send(BotRequest::GetRolesOfUser(member.user.id)).await?
        else {
            bail!(RouterError::InvalidResponse)
        };

        roles
    };

    // find user id database, and fetch roles
    let prisma_user = prisma
        .user()
        .find_unique(user::id::equals(member.user.id))
        .with(user::roles::fetch(vec![]))
        .exec()
        .await?
        .make_error(MemberLogError::NotFound(member.user.id))?;

    // find user's color role in db
    let color_role = prisma_user
        .roles()?
        .iter()
        .filter(|r| r.color_role)
        .next()
        .make_error(MemberLogError::NoColorRole(member.user.id))?;

    // if user had removed the color role, just re-add it
    if !roles.iter().any(|role| i64::from(role.id) == color_role.id) {
        bot.send(BotRequest::AddRoleToUser(
            member.user.id,
            serenity::RoleId::new(color_role.id as u64),
        ))
        .await?;
    }

    // every role out of sync with the database -- to add and remove
    let mut connects = vec![];
    let mut disconnects = vec![];

    // for every role the user has
    for role in &roles {
        // both custom and blacklisted roles are ignored
        if !super::role::log_check(role.id).await {
            continue;
        }

        // if the user is verified, add the verified role
        if role.id == nci::roles::MEMBERS {
            updates.push(user::verified::set(true));
        }

        // if the user is an admin, add the admin role
        if role.id == nci::roles::OVERRIDES {
            updates.push(user::admin::set(true));
        }

        // if the database has not registered the role, add it
        if !prisma_user
            .roles()?
            .iter()
            .any(|r| i64::from(role.id) == r.id)
        {
            connects.push(role::id::equals(role.id));
        }
    }

    // if user removed roles, add to db
    for role in prisma_user.roles()? {
        if !super::role::log_check(role.id as u64).await {
            continue;
        }

        // if the user is verified, update verified status
        if role.id == i64::from(nci::roles::MEMBERS)
            && !roles.iter().any(|role| role.id == nci::roles::MEMBERS)
        {
            updates.push(user::verified::set(false));
            disconnects.push(role::id::equals(role.id));
        }

        // if the user is an admin, update admin status
        if role.id == i64::from(nci::roles::OVERRIDES)
            && !roles.iter().any(|role| role.id == nci::roles::OVERRIDES)
        {
            updates.push(user::admin::set(false));
            disconnects.push(role::id::equals(role.id));
        }

        // if the user no longer has the role, unlink in db
        if !roles.iter().any(|r| i64::from(r.id) == role.id) {
            disconnects.push(role::id::equals(role.id));
        }
    }

    // update user's bot status and role (linked to member.user.bot)
    if member.user.bot {
        if !prisma_user.bot {
            updates.push(user::bot::set(true));
        }

        if !prisma_user
            .roles()?
            .iter()
            .any(|r| r.id == i64::from(nci::roles::BOTS))
        {
            connects.push(role::id::equals(nci::roles::BOTS));
        }

        if !member.roles.iter().any(|r| *r == nci::roles::BOTS) {
            bot.send(BotRequest::AddRoleToUser(member.user.id, nci::roles::BOTS))
                .await?;
        }
    } else {
        if prisma_user.bot {
            updates.push(user::bot::set(false));
        }

        if prisma_user
            .roles()?
            .iter()
            .any(|r| r.id == i64::from(nci::roles::BOTS))
        {
            disconnects.push(role::id::equals(nci::roles::BOTS));
        }

        if roles.iter().any(|r| r.id == nci::roles::BOTS) {
            bot.send(BotRequest::RemoveRoleFromUser(
                member.user.id,
                nci::roles::BOTS,
            ))
            .await?;
        }
    }

    updates.push(user::roles::connect(connects));
    updates.push(user::roles::disconnect(disconnects));

    // update other stuff
    if member.user.name != prisma_user.username {
        updates.push(user::username::set(member.user.name));
    }

    if member.nick != prisma_user.nickname {
        updates.push(user::nickname::set(member.nick));
    }

    prisma
        .user()
        .update(user::id::equals(member.user.id), updates)
        .exec()
        .await?;

    Ok(())
}

pub async fn delete(
    id: serenity::UserId,
    bot: &mut Client<BotServer>,
) -> Result<(), MemberLogError> {
    let prisma = prisma::create().await?;

    let prisma_user = prisma
        .user()
        .find_unique(user::id::equals(id))
        .with(user::roles::fetch(vec![role::color_role::equals(true)]))
        .exec()
        .await?
        .make_error(MemberLogError::NotFound(id))?;

    let color_role_id = serenity::RoleId::new(
        prisma_user
            .roles()?
            .first()
            .make_error(MemberLogError::NoColorRole(id))?
            .id as u64,
    );

    prisma
        .user()
        .update(
            user::id::equals(id),
            vec![user::removed::set(true), user::roles::set(vec![])],
        )
        .exec()
        .await?;

    bot.send(BotRequest::DeleteRole(color_role_id)).await?;

    Ok(())
}

pub async fn read(id: serenity::UserId) -> Result<prisma::data::UserData, MemberLogError> {
    let prisma = prisma::create().await?;

    let user = prisma
        .user()
        .find_unique(user::id::equals(id))
        .with(user::roles::fetch(vec![]))
        .with(user::messages::fetch(vec![]))
        .with(user::interactions::fetch(vec![]))
        .with(user::settings::fetch())
        .exec()
        .await?
        .make_error(MemberLogError::NotFound(id))?;

    Ok(user)
}
