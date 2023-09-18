use crate::prelude::*;

async fn roles(nci: &serenity::Guild, prisma: &PrismaClient) -> Result<(), LoggingError> {
    // fetch all roles from discord and the database
    let roles = nci.roles.values().collect::<Vec<_>>();
    let prisma_roles = prisma.role().find_many(vec![]).exec().await?;

    for role in &roles {
        if let Some(prisma_role) = prisma_roles.iter().find(|r| r.id == role.id.to_string()) {
            // update role if exists
            let mut updates = vec![];

            if role.name != prisma_role.name {
                updates.push(role::name::set(role.name.clone()));
            }

            if role.colour.hex() != prisma_role.color {
                updates.push(role::color::set(role.colour.hex()));
            }

            if nci::roles::is_color_role(role.id) != prisma_role.color_role {
                updates.push(role::color_role::set(nci::roles::is_color_role(role.id)));
            }

            // if no updates, skip
            if updates.is_empty() {
                continue;
            }

            prisma
                .role()
                .update(role::id::equals(role.id.to_string()), updates)
                .exec()
                .await?;

            if nci::roles::can_log(serenity::RoleId(u64::from_str(&role.id.to_string())?)) {
                prisma
                    .role()
                    .delete(role::id::equals(role.id.to_string()))
                    .exec()
                    .await?;
            }
        } else {
            // otherwise create it
            prisma
                .role()
                .create(
                    role.id.to_string(),
                    role.name.clone(),
                    role.colour.hex(),
                    vec![role::color_role::set(nci::roles::is_color_role(role.id))],
                )
                .exec()
                .await?;
        }
    }

    // check for roles in db that don't exist in discord
    let mut remove = vec![];

    for role in &prisma_roles {
        if roles.iter().find(|n| n.id.to_string() == role.id).is_none() && !role.deleted {
            remove.push(role::id::equals(role.id.clone()));
        }
    }

    if !remove.is_empty() {
        prisma
            .role()
            .update_many(remove, vec![role::deleted::set(true)])
            .exec()
            .await?;
    }

    Ok(())
}

async fn users(
    ctx: &serenity::Context,
    nci: &serenity::Guild,
    prisma: &PrismaClient,
) -> Result<(), LoggingError> {
    let mut users = nci.members.values().cloned().collect::<Vec<_>>();
    let prisma_users = prisma
        .user()
        .find_many(vec![])
        .with(user::roles::fetch(vec![]))
        .exec()
        .await?;

    let mut color_roles = vec![];

    for member in &mut users {
        let mut member_roles = vec![];

        // get or create the user's color role
        let color_role = match member.roles.iter().find(|r| nci::roles::is_color_role(**r)) {
            Some(n) => nci.roles.get(n).cloned().unwrap(),
            None => {
                let role = nci
                    .create_role(&ctx, |r| {
                        r.clone_from(&default_role(member));
                        r
                    })
                    .await?;

                member.add_role(&ctx, role.id).await?;

                prisma
                    .role()
                    .create(
                        role.id.to_string(),
                        role.name.clone(),
                        role.colour.hex(),
                        vec![role::color_role::set(true), role::users::set(vec![])],
                    )
                    .exec()
                    .await?;

                role
            }
        };

        for role in &member.roles {
            member_roles.push(role.to_string());
        }

        // update or create the user in the database
        if let Some(prisma_user) = prisma_users
            .iter()
            .find(|u| u.id == member.user.id.to_string())
        {
            let mut updates = vec![];

            if member.user.name != prisma_user.username {
                updates.push(user::username::set(member.user.name.clone()));
            }

            if member.nick != prisma_user.nickname {
                updates.push(user::nickname::set(member.nick.clone()));
            }

            if member.roles.contains(&nci::roles::OVERRIDES) != prisma_user.admin {
                updates.push(user::admin::set(!prisma_user.admin));
            }

            if member.roles.contains(&nci::roles::MEMBERS) != prisma_user.verified {
                updates.push(user::verified::set(!prisma_user.verified));
            }

            if prisma_user.removed {
                updates.push(user::removed::set(false));
            }

            if member_roles
                != prisma_user
                    .roles
                    .clone()
                    .unwrap_or(vec![])
                    .iter()
                    .map(|n| &n.id)
                    .cloned()
                    .collect::<Vec<_>>()
            {
                updates.push(user::roles::set(
                    member_roles
                        .iter()
                        .map(|n| role::id::equals(n.clone()))
                        .collect::<Vec<_>>(),
                ));
            }

            if !updates.is_empty() {
                prisma
                    .user()
                    .update(user::id::equals(member.user.id.to_string()), updates)
                    .exec()
                    .await?;
            }
        } else {
            prisma
                .user()
                .create(
                    member.user.id.to_string(),
                    member.user.name.clone(),
                    vec![
                        user::nickname::set(member.nick.clone()),
                        user::admin::set(member.roles.contains(&nci::roles::OVERRIDES)),
                        user::verified::set(member.roles.contains(&nci::roles::MEMBERS)),
                        user::bot::set(member.user.bot),
                        user::roles::connect(
                            member_roles
                                .iter()
                                .map(|n| role::id::equals(n.clone()))
                                .collect::<Vec<_>>(),
                        ),
                    ],
                )
                .exec()
                .await?;
        }

        // update the list of color roles
        color_roles.push(role::id::equals(color_role.id.to_string()));
    }

    prisma
        .role()
        .update_many(color_roles, vec![role::color_role::set(true)])
        .exec()
        .await?;

    // check for users in db that don't exist in discord
    let mut remove = vec![];

    for user in &prisma_users {
        if users
            .iter()
            .find(|n| n.user.id.to_string() == user.id)
            .is_none()
            && !user.removed
        {
            remove.push(user::id::equals(user.id.clone()));
        }
    }

    if !remove.is_empty() {
        prisma
            .user()
            .update_many(remove, vec![user::removed::set(true)])
            .exec()
            .await?;
    }

    Ok(())
}

async fn categories(nci: &serenity::Guild, prisma: &PrismaClient) -> Result<(), LoggingError> {
    let prisma_categories = prisma.channel_category().find_many(vec![]).exec().await?;
    let categories = nci
        .channels
        .values()
        .filter_map(|c| c.clone().category())
        .collect::<Vec<_>>();

    for category in &categories {
        if let Some(prisma_category) = prisma_categories
            .iter()
            .find(|n| n.id == category.id.to_string())
        {
            let mut updates = vec![];

            if prisma_category.name != category.name {
                updates.push(channel_category::name::set(category.name.clone()));
            }

            if !updates.is_empty() {
                prisma
                    .channel_category()
                    .update(
                        channel_category::id::equals(category.id.to_string()),
                        updates,
                    )
                    .exec()
                    .await?;
            }
        } else {
            prisma
                .channel_category()
                .create(category.id.to_string(), category.name.clone(), vec![])
                .exec()
                .await?;
        }
    }

    // check for categories in db that don't exist in discord
    let mut remove = vec![];

    for category in &prisma_categories {
        if categories
            .iter()
            .find(|n| n.id.to_string() == category.id)
            .is_none()
            && !category.deleted
        {
            remove.push(channel_category::id::equals(category.id.clone()));
        }
    }

    if !remove.is_empty() {
        prisma
            .channel_category()
            .update_many(remove, vec![channel_category::deleted::set(true)])
            .exec()
            .await?;
    }

    Ok(())
}

async fn channels(nci: &serenity::Guild, prisma: &PrismaClient) -> Result<(), LoggingError> {
    let prisma_channels = prisma.channel().find_many(vec![]).exec().await?;
    let channels = nci
        .channels
        .values()
        .filter_map(|v| v.clone().guild())
        .collect::<Vec<_>>();

    for channel in &channels {
        if channel.is_thread() {
            continue;
        }

        if let Some(prisma_channel) = prisma_channels
            .iter()
            .find(|n| n.id == channel.id.to_string())
        {
            let mut updates = vec![];

            if prisma_channel.name != channel.name {
                updates.push(channel::name::set(channel.name.clone()));
            }

            if channel.topic != prisma_channel.topic {
                updates.push(channel::topic::set(channel.topic.clone()));
            }

            if channel.nsfw != prisma_channel.nsfw {
                updates.push(channel::nsfw::set(channel.nsfw));
            }

            if channel.kind
                != match prisma_channel.kind {
                    ChannelType::Text => serenity::ChannelType::Text,
                    ChannelType::News => serenity::ChannelType::News,
                    ChannelType::Stage => serenity::ChannelType::Stage,
                    ChannelType::Voice => serenity::ChannelType::Voice,
                }
            {
                updates.push(channel::kind::set(match channel.kind {
                    serenity::ChannelType::Text => ChannelType::Text,
                    serenity::ChannelType::News => ChannelType::News,
                    serenity::ChannelType::Stage => ChannelType::Stage,
                    serenity::ChannelType::Voice => ChannelType::Voice,
                    _ => return Ok(()), /* Other channels should not be logged */
                }));
            }

            if channel.parent_id.map(|n| n.to_string()) != prisma_channel.category_id {
                if let Some(parent_id) = channel.parent_id {
                    updates.push(channel::category::connect(channel_category::id::equals(
                        parent_id.to_string(),
                    )));
                } else {
                    updates.push(channel::category::disconnect());
                }
            }

            if !updates.is_empty() {
                prisma
                    .channel()
                    .update(channel::id::equals(channel.id.to_string()), updates)
                    .exec()
                    .await?;
            }
        } else {
            prisma
                .channel()
                .create(
                    channel.id.to_string(),
                    channel.name.clone(),
                    channel.nsfw,
                    match channel.kind {
                        serenity::ChannelType::Text => ChannelType::Text,
                        serenity::ChannelType::News => ChannelType::News,
                        serenity::ChannelType::Stage => ChannelType::Stage,
                        serenity::ChannelType::Voice => ChannelType::Voice,
                        _ => return Ok(()), /* Other channels should not be logged */
                    },
                    {
                        /* omg i love blocks */
                        let mut params = vec![channel::topic::set(channel.topic.clone())];

                        if let Some(parent_id) = channel.parent_id {
                            params.push(channel::category::connect(channel_category::id::equals(
                                parent_id.to_string(),
                            )));
                        }

                        params
                    },
                )
                .exec()
                .await?;
        }
    }

    // check for channels in db that don't exist in discord
    let mut remove = vec![];

    for channel in &prisma_channels {
        if channels
            .iter()
            .find(|n| n.id.to_string() == channel.id)
            .is_none()
            && !channel.deleted
        {
            remove.push(channel::id::equals(channel.id.clone()));
        }
    }

    if !remove.is_empty() {
        prisma
            .channel()
            .update_many(remove.clone(), vec![channel::deleted::set(true)])
            .exec()
            .await?;

        for channel in remove {
            prisma
                .message()
                .update_many(
                    vec![message::channel::is(vec![channel])],
                    vec![message::deleted::set(true)],
                )
                .exec()
                .await?;
        }
    }

    Ok(())
}

pub async fn on_ready(ctx: serenity::Context) -> Result<(), LoggingError> {
    // find NCI server
    let nci_id = *ctx
        .cache
        .guilds()
        .iter()
        .find(|g| g.to_string() == nci::ID.to_string())
        .make_error(LoggingError::NciNotFound)?;

    let nci = ctx
        .cache
        .guild(nci_id)
        .make_error(LoggingError::NciNotFound)?;

    let prisma = prisma::create().await?;

    roles(&nci, &prisma).await?;
    users(&ctx, &nci, &prisma).await?;
    categories(&nci, &prisma).await?;
    channels(&nci, &prisma).await?;

    Ok(())
}
