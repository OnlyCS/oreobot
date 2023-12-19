use crate::prelude::*;

async fn roles(bot: &mut Client<BotServer>) -> Result<(), RoleLogError> {
    let prisma = prisma::create().await?;

    let roles = {
        let BotResponse::RolesOk(roles) = bot.send(BotRequest::GetAllRoles).await? else {
            bail!(RouterError::InvalidResponse)
        };

        roles.into_iter().fold(HashMap::new(), |mut collect, item| {
            collect.insert(item.id, item);
            collect
        })
    };

    let prisma_roles = prisma
        .role()
        .find_many(vec![])
        .exec()
        .await?
        .into_iter()
        .map(|n| n.id)
        .collect::<HashSet<_>>();

    // for every role in the database
    for id_i64 in &prisma_roles {
        let id = serenity::RoleId::new(*id_i64 as u64);

        if let Some(role) = roles.get(&id) {
            super::role::update(role.clone()).await?;
        } else {
            super::role::delete(id, bot).await?;
        }
    }

    // for every role in the guild
    for (id, role) in roles {
        if !super::role::log_check(id).await {
            continue;
        }

        if !prisma_roles.contains(&id.into()) {
            super::role::create(role).await?;
        }
    }

    Ok(())
}

async fn members(bot: &mut Client<BotServer>) -> Result<(), MemberLogError> {
    debug!("got here");

    let prisma = prisma::create().await?;

    let members = {
        let BotResponse::MembersOk(members) = bot.send(BotRequest::GetAllMembers).await? else {
            bail!(RouterError::InvalidResponse)
        };

        members
            .into_iter()
            .fold(HashMap::new(), |mut collect, item| {
                collect.insert(item.user.id, item);
                collect
            })
    };

    let prisma_members = prisma
        .user()
        .find_many(vec![])
        .with(user::roles::fetch(vec![]))
        .exec()
        .await?
        .into_iter()
        .map(|n| n.id)
        .collect::<HashSet<_>>();

    for (id, member) in &members {
        if prisma_members.contains(&i64::from(*id)) {
            let member_str = serde_json::to_string(&member).unwrap();
            let event = serde_json::from_str(&member_str).unwrap(); /* same fields */

            super::member::update(event, bot).await?;
        } else {
            super::member::create(member.clone(), bot).await?;
        }

        // get the user's settings so it will create if dne
        super::user_settings::read(*id).await?;
    }

    for id_i64 in prisma_members {
        let id = serenity::UserId::new(id_i64 as u64);

        if !members.contains_key(&id) {
            super::member::delete(id, bot).await?;
        }
    }

    Ok(())
}

async fn categories(bot: &mut Client<BotServer>) -> Result<(), CategoryLogError> {
    let prisma = prisma::create().await?;

    let categories = {
        let BotResponse::CategoriesOk(categories) = bot.send(BotRequest::GetAllCategories).await?
        else {
            bail!(RouterError::InvalidResponse)
        };

        categories
            .into_iter()
            .fold(HashMap::new(), |mut collect, item| {
                collect.insert(item.id, item);
                collect
            })
    };

    let prisma_categories = prisma
        .channel_category()
        .find_many(vec![])
        .exec()
        .await?
        .into_iter()
        .map(|n| n.id)
        .collect::<HashSet<_>>();

    for (id, category) in &categories {
        if prisma_categories.contains(&i64::from(*id)) {
            super::category::update(category.clone()).await?;
        } else {
            super::category::create(category.clone()).await?;
        }
    }

    debug!("checkpoint1");

    for id_i64 in prisma_categories {
        let id = serenity::ChannelId::new(id_i64 as u64);

        if !categories.contains_key(&id) {
            super::category::delete(id).await?;
        }
    }

    debug!("checkpoint2");

    Ok(())
}

async fn channels(bot: &mut Client<BotServer>) -> Result<(), ChannelLogError> {
    let prisma = prisma::create().await?;

    let channels = {
        let BotResponse::ChannelsOk(channels) = bot.send(BotRequest::GetAllChannels).await? else {
            bail!(RouterError::InvalidResponse)
        };

        channels
            .into_iter()
            .fold(HashMap::new(), |mut collect, item| {
                collect.insert(item.id, item);
                collect
            })
    };

    let prisma_channels = prisma
        .channel()
        .find_many(vec![])
        .exec()
        .await?
        .into_iter()
        .map(|n| n.id)
        .collect::<HashSet<_>>();

    for (id, channel) in channels.clone() {
        if prisma_channels.contains(&id.into()) {
            super::channel::update(channel.clone()).await?;
        } else {
            super::channel::create(channel.clone()).await?;
        }
    }

    debug!("checkpoint3");

    for id_i64 in prisma_channels {
        let id = serenity::ChannelId::new(id_i64 as u64);

        if !channels.contains_key(&id) {
            super::channel::delete(id).await?;
        }
    }

    debug!("checkpoint4");

    Ok(())
}

pub async fn ready(bot: &mut Client<BotServer>) -> Result<(), ReadyEventError> {
    roles(bot).await?;

    debug!("Roles done");

    members(bot).await?;

    debug!("Members done");

    categories(bot).await?;

    debug!("Categories done");

    channels(bot).await?;

    debug!("Channels done");

    Ok(())
}
