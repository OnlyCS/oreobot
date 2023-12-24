use crate::prelude::*;

pub async fn join(
    member: serenity::Member,
    bot: &mut Client<BotServer>,
) -> Result<(), MemberLogError> {
    let prisma = prisma::create().await?;

    // try to find a user that already exists
    let old_user = prisma
        .user()
        .find_unique(user::id::equals(member.user.id))
        .with(user::roles::fetch(vec![]))
        .exec()
        .await?;

    // must create user in db before trying anything that triggers member update
    if old_user.is_none() {
        prisma
            .user()
            .create(
                member.user.id,
                &member.user.name,
                vec![
                    user::display_name::set(member.user.global_name.as_ref().cloned()),
                    user::nickname::set(member.nick),
                ],
            )
            .exec()
            .await?;
    }

    // create the user's color role
    // wants the number of custom roles
    let custom_roles = prisma
        .role()
        .find_many(vec![role::kind::equals(RoleType::CustomRole)])
        .exec()
        .await?
        .len() as u16;

    bot.send(BotRequest::CreateColorRole {
        user_id: member.user.id,
        custom_roles,
    })
    .await?;

    // give the user their old roles back
    if let Some(old_user) = old_user {
        for role in old_user.roles()? {
            bot.send(BotRequest::AddRoleToUser(
                member.user.id,
                serenity::RoleId::new(role.id as u64),
            ))
            .await?;
        }

        prisma
            .user()
            .update(
                user::id::equals(member.user.id),
                vec![user::removed::set(false)],
            )
            .exec()
            .await?;
    }

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
        .exec()
        .await?
        .make_error(MemberLogError::NotFound(id))?;

    Ok(user)
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
        .filter(|r| r.kind == RoleType::ColorRole)
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
        // if the user no longer has the role, unlink in db
        if !roles.iter().any(|r| i64::from(r.id) == role.id) {
            disconnects.push(role::id::equals(role.id));
        }
    }

    // update user's bot status and role (linked to member.user.bot)
    if member.user.bot {
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
    if member.user.name != prisma_user.name {
        updates.push(user::name::set(member.user.name));
    }

    if member.user.global_name != prisma_user.display_name {
        updates.push(user::display_name::set(member.user.global_name))
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

pub async fn leave(
    id: serenity::UserId,
    bot: &mut Client<BotServer>,
) -> Result<(), MemberLogError> {
    let prisma = prisma::create().await?;

    let prisma_user = prisma
        .user()
        .find_unique(user::id::equals(id))
        .with(user::roles::fetch(vec![role::kind::equals(
            RoleType::ColorRole,
        )]))
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
        .update(user::id::equals(id), vec![user::removed::set(true)])
        .exec()
        .await?;

    bot.send(BotRequest::DeleteRole(color_role_id)).await?;

    Ok(())
}
