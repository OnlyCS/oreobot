use crate::prelude::*;

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

    // setup prisma
    let prisma_mutex = Arc::clone(
        ctx.data
            .read()
            .await
            .get::<prisma::PrismaTypeKey>()
            .context("Could not find prisma")?,
    );
    let prisma = prisma_mutex.lock().await;

    // collect members and users/roles in database
    let members = nci.members.values().collect::<Vec<_>>();
    let prisma_members = prisma.user().find_many(vec![]).exec().await?;
    let prisma_roles = prisma.user_role().find_many(vec![]).exec().await?;

    for member in members {
        // dont update db for bots
        if member.user.bot {
            continue;
        }

        // find the member in the database, if they exist
        let id = member.user.id.to_string();
        let prisma_member = prisma_members.iter().find(|m| m.id == id);

        // find or create their user role
        let option_role_id = member.roles.iter().find(|r| {
            r.to_string() != nci::roles::OVERRIDES && r.to_string() != nci::roles::MEMBERS
        });

        let role;

        if let Some(role_id) = option_role_id {
            role = ctx
                .cache
                .role(nci_id, role_id)
                .context("Could not find role")?;
        } else {
            role = nci
                .create_role(&ctx, |r| {
                    r.name(&member.nick.as_ref().unwrap_or(&member.user.name))
                        .colour(serenity::Color::RED.0.into())
                        .position(5)
                        .mentionable(false)
                })
                .await?;

            ctx.cache
                .member(nci_id, member.user.id)
                .as_mut()
                .context("Could not find member")?
                .add_role(&ctx, role.id)
                .await?;
        }

        // if role not found in db, create
        if prisma_roles
            .iter()
            .find(|r| r.id == role.id.to_string())
            .is_none()
        {
            prisma
                .user_role()
                .create(role.id.to_string(), role.name, role.colour.hex(), vec![])
                .exec()
                .await?;
        }

        if let Some(prisma_member) = prisma_member {
            // if user found in db, update to match current info
            let mut updates_user = vec![];

            if member.user.name != prisma_member.username {
                updates_user.push(user::username::set(member.user.name.clone()));
            }

            if member.nick.is_some() && &member.nick != &prisma_member.nickname {
                updates_user.push(user::nickname::set(member.nick.clone()));
            } else if member.nick.is_none() && prisma_member.nickname.is_some() {
                updates_user.push(user::nickname::set(None));
            }

            if role.id.to_string() != prisma_member.role_id {
                updates_user.push(user::role::connect(user_role::id::equals(
                    role.id.to_string(),
                )));
            }

            if member
                .roles
                .iter()
                .map(|r| r.to_string())
                .any(|r| r == nci::roles::OVERRIDES)
                != prisma_member.admin
            {
                updates_user.push(user::admin::set(!prisma_member.admin));
            }

            if !updates_user.is_empty() {
                prisma
                    .user()
                    .update(user::id::equals(member.user.id.to_string()), updates_user)
                    .exec()
                    .await?;
            }
        } else {
            // if not, create user in db
            prisma
                .user()
                .create(
                    member.user.id.to_string(),
                    member.user.name.clone(),
                    user_role::id::equals(role.id.to_string()),
                    vec![
                        user::admin::set(
                            member
                                .roles
                                .iter()
                                .map(|n| n.to_string())
                                .any(|n| n == nci::roles::OVERRIDES),
                        ),
                        user::nickname::set(member.nick.clone()),
                    ],
                )
                .exec()
                .await?;
        }
    }

    // get all channels and categories in server and db
    let mut channels = nci.channels.values().collect::<Vec<_>>();

    let categories = channels
        .extract_if(|c| c.clone().category().is_some())
        .collect::<Vec<_>>();

    let prisma_channels = prisma.channel().find_many(vec![]).exec().await?;
    let prisma_categories = prisma.channel_category().find_many(vec![]).exec().await?;

    for category in categories {
        let category = category
            .clone()
            .category()
            .context("Filtering was unsucesful")?;

        // find the category in the database, if it exists
        if let Some(prisma_category) = prisma_categories
            .iter()
            .find(|c| c.id == category.id.to_string())
        {
            if prisma_category.name != category.name {
                prisma
                    .channel_category()
                    .update(
                        channel_category::id::equals(category.id.to_string()),
                        vec![channel_category::name::set(category.name.clone())],
                    )
                    .exec()
                    .await?;
            }
        } else {
            // if not, create category in db
            info!("About to create channel category");
            prisma
                .channel_category()
                .create(category.id.to_string(), category.name.clone(), vec![])
                .exec()
                .await?;
        }
    }

    for channel in channels {
        let channel = channel.clone().guild().context("Could not find channel")?;

        // find the channel in the database, if it exists
        if let Some(prisma_channel) = prisma_channels
            .iter()
            .find(|c| c.id == channel.id.to_string())
        {
            let mut channel_updates = vec![];

            if channel.name != prisma_channel.name {
                channel_updates.push(channel::name::set(channel.name.clone()));
            }

            if let Some(parent_id) = channel.parent_id
                && Some(parent_id.to_string()) != prisma_channel.category_id
            {
				channel_updates.push(channel::category::connect(channel_category::id::equals(parent_id.to_string())))
            }

            if let Some(channel_topic) = channel.topic.as_ref() {
                channel_updates.push(channel::topic::set(Some(channel_topic.to_string())));
            }

            if prisma_channel.nsfw == channel.is_nsfw() {
                channel_updates.push(channel::nsfw::set(channel.is_nsfw()));
            }

            match channel.kind {
                serenity::ChannelType::News if prisma_channel.kind != PChannelType::News => {
                    channel_updates.push(channel::kind::set(PChannelType::News))
                }
                serenity::ChannelType::Text if prisma_channel.kind != PChannelType::Text => {
                    channel_updates.push(channel::kind::set(PChannelType::Text))
                }
                serenity::ChannelType::Stage if prisma_channel.kind != PChannelType::Stage => {
                    channel_updates.push(channel::kind::set(PChannelType::Stage))
                }
                serenity::ChannelType::Voice if prisma_channel.kind != PChannelType::Voice => {
                    channel_updates.push(channel::kind::set(PChannelType::Voice))
                }
                _ => continue,
            }

            if !channel_updates.is_empty() {
                prisma
                    .channel()
                    .update(channel::id::equals(channel.id.to_string()), channel_updates)
                    .exec()
                    .await?;
            }
        } else {
            // else, create
            info!("about to create channel");
            prisma
                .channel()
                .create(
                    channel.id.to_string(),
                    channel.name.clone(),
                    channel.is_nsfw(),
                    match channel.kind {
                        serenity::ChannelType::News => PChannelType::News,
                        serenity::ChannelType::Text => PChannelType::Text,
                        serenity::ChannelType::Stage => PChannelType::Stage,
                        serenity::ChannelType::Voice => PChannelType::Voice,
                        _ => continue,
                    },
                    if let Some(parent_id) = channel.parent_id {
                        vec![channel::category::connect(channel_category::id::equals(
                            parent_id.to_string(),
                        ))]
                    } else {
                        vec![]
                    },
                )
                .exec()
                .await?;
        }
    }

    Ok(())
}
