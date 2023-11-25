use crate::prelude::*;

pub async fn create(mut member: serenity::Member) -> Result<(), MemberLogError> {
    let prisma = prisma::create().await?;

    let color_role: serenity::Role = todo!("Create user's color role (tcp with bot). Should trigger rolecreate event, handled in role.rs");
    let mut roles = vec![color_role.id];

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

        todo!("add roles[] to user")
    } else {
        prisma
            .user()
            .create(
                member.user.id,
                member.user.name,
                vec![
                    user::nickname::set(member.nick),
                    user::roles::set(roles.iter().map(|n| role::id::equals(*n)).collect_vec()),
                    user::bot::set(member.user.bot),
                ],
            )
            .exec()
            .await?;
    }

    Ok(())
}

pub async fn update(member: serenity::Member) -> Result<(), MemberLogError> {
    let prisma = prisma::create().await?;
    let mut updates = vec![];

    let roles: Vec<serenity::Role> = todo!("Comms: get roles of user");

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
        todo!("Comms: user add role");
    }

    // if user has new roles, add to db
    let mut connects = vec![];
    for role in roles {
        if !nci::roles::can_log(role.id) {
            continue;
        }

        if role.id == nci::roles::MEMBERS {
            updates.push(user::verified::set(true));
        }

        if role.id == nci::roles::OVERRIDES {
            updates.push(user::admin::set(true));
        }

        if role.id == nci::roles::BOTS && !member.user.bot {
            todo!("Comms: user not a bot, may have been added by mistake, remove role from user");
        }

        if !prisma_user
            .roles()?
            .iter()
            .filter(|r| !r.color_role)
            .any(|r| i64::from(role.id) == r.id)
        {
            connects.push(role::id::equals(role.id));
        }
    }
    updates.push(user::roles::connect(connects));

    // if user removed roles, add to db
    let mut disconnects = vec![];
    for role in prisma_user.roles()? {
        if !nci::roles::can_log(role.id) {
            continue;
        }

        if role.id == i64::from(nci::roles::MEMBERS) {
            updates.push(user::verified::set(false));
        }

        if role.id == i64::from(nci::roles::OVERRIDES) {
            updates.push(user::admin::set(false));
        }

        if role.id == i64::from(nci::roles::BOTS) && member.user.bot {
            todo!("Comms: user is a bot, may have been removed by mistake, re-add role from user");
        }

        if !roles.iter().any(|r| i64::from(r.id) == role.id) {
            disconnects.push(role::id::equals(role.id));
        }
    }
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

pub async fn delete(id: serenity::UserId) -> Result<(), MemberLogError> {
    let prisma = prisma::create().await?;

    let prisma_user = prisma
        .user()
        .find_unique(user::id::equals(id))
        .with(user::roles::fetch(vec![role::color_role::equals(true)]))
        .exec()
        .await?
        .make_error(MemberLogError::NotFound(id))?;

    let color_role_id = prisma_user
        .roles()?
        .first()
        .make_error(MemberLogError::NoColorRole(id))?
        .id;

    prisma
        .user()
        .update(
            user::id::equals(id),
            vec![user::removed::set(true), user::roles::set(vec![])],
        )
        .exec()
        .await?;

    todo!("Comms: delete color role");

    Ok(())
}

pub async fn read(id: serenity::UserId) -> Result<prisma::data::UserData, MemberLogError> {
    let prisma = prisma::create().await?;

    let user = prisma
        .user()
        .find_unique(user::id::equals(id))
        .with(user::roles::fetch(vec![]))
        .with(user::messages::fetch(vec![]))
        .with(user::impersonated_messages::fetch(vec![]))
        .with(user::interactions::fetch(vec![]))
        .with(user::settings::fetch())
        .exec()
        .await?
        .make_error(MemberLogError::NotFound(id))?;

    Ok(user)
}
