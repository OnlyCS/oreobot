use crate::prelude::*;

pub async fn roles(nci: &serenity::Guild, prisma: &PrismaClient) -> Result<()> {
    // fetch all roles from discord and the database
    let roles = nci.roles.values().collect::<Vec<_>>();
    let prisma_roles = prisma.role().find_many(vec![]).exec().await?;

    for role in roles {
        if let Some(prisma_role) = prisma_roles.iter().find(|r| r.id == role.id.to_string()) {
            // update role if exists
            let mut updates = vec![];

            if role.name != prisma_role.name {
                updates.push(role::name::set(role.name.clone()));
            }

            if role.colour.hex() != prisma_role.color {
                updates.push(role::color::set(role.colour.hex()));
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
        } else {
            // otherwise create it
            prisma
                .role()
                .create(
                    role.id.to_string(),
                    role.name.clone(),
                    role.colour.hex(),
                    vec![],
                )
                .exec()
                .await?;
        }
    }

    Ok(())
}

pub async fn users(nci: &serenity::Guild, prisma: &PrismaClient) -> Result<()> {
    let members = nci.members.values().collect::<Vec<_>>();
    let prisma_members = prisma.user().find_many(vec![]).exec().await?;

    // to update for color roles
    let mut color_roles = vec![];

    for member in members {
        let mut updates = vec![];
        let mut roles = vec![];

        if let Some(prisma_member) = prisma_members
            .iter()
            .find(|m| m.id == member.user.id.to_string())
        {
            // update user if exists
            if prisma_member.username != member.user.name {
                updates.push(user::username::set(member.user.name.clone()));
            }

            if prisma_member.nickname != member.nick {
                updates.push(user::nickname::set(member.nick.clone()));
            }
        } else {
            // otherwise create it
            prisma
                .user()
                .create(
                    member.user.id.to_string(),
                    member.user.name.clone(),
                    vec![
                        user::nickname::set(member.nick.clone()),
                        user::bot::set(member.user.bot),
                    ],
                )
                .exec()
                .await?;
        }

        // update user roles
        let role_ids = &member.roles;

        for role_id in role_ids {
            if role_id.to_string() == nci::roles::OVERRIDES {
                updates.push(user::admin::set(true));
            } else if role_id.to_string() == nci::roles::MEMBERS {
                updates.push(user::verified::set(true));
            } else if role_id.to_string() != nci::roles::BOTS {
                updates.push(user::color_role_id::set(Some(role_id.to_string())));
                color_roles.push(role::id::equals(role_id.to_string()));
            }

            roles.push(role::id::equals(role_id.to_string()));
        }

        updates.push(user::roles::connect(roles));

        // push all updates
        prisma
            .user()
            .update(user::id::equals(member.user.id.to_string()), updates)
            .exec()
            .await?;
    }

    // push all color roles to db
    prisma
        .role()
        .update_many(color_roles, vec![role::is_color_role::set(true)])
        .exec()
        .await?;

    Ok(())
}

pub async fn categories(nci: &serenity::Guild, prisma: &PrismaClient) -> Result<()> {
    let prisma_categories = prisma.channel_category().find_many(vec![]).exec().await?;
    let categories = nci
        .channels
        .values()
        .filter_map(|c| c.clone().category())
        .collect::<Vec<_>>();

    for category in categories {
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

    Ok(())
}

pub async fn channels(nci: &serenity::Guild, prisma: &PrismaClient) -> Result<()> {
    let prisma_channels = prisma.channel().find_many(vec![]).exec().await?;
    let channels = nci
        .channels
        .values()
        .filter_map(|v| v.clone().guild())
        .collect::<Vec<_>>();

    for channel in channels {
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
                        let mut params = vec![channel::topic::set(channel.topic)];

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

    Ok(())
}

pub async fn ready(ctx: serenity::Context) -> Result<()> {
    // find NCI server
    let nci_id = ctx
        .cache
        .guilds()
        .iter()
        .find(|g| g.to_string() == nci::ID.to_string())
        .context("Could not find NCI by id")?
        .clone();

    let nci = ctx
        .cache
        .guild(nci_id)
        .context("Could not find NCI server data")?;

    get_prisma::from_serenity_context!(prisma, ctx);

    users(&nci, &prisma).await?;
    roles(&nci, &prisma).await?;
    categories(&nci, &prisma).await?;
    channels(&nci, &prisma).await?;

    Ok(())
}
