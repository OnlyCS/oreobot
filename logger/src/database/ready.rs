use crate::prelude::prisma_client_rust;
use crate::prelude::*;
use std::collections::{HashMap, HashSet};

async fn roles() -> Result<(), RoleLogError> {
    let prisma = prisma::create().await?;
    let roles: HashMap<serenity::RoleId, serenity::Role> = todo!("Comms: Get list of roles");

    let prisma_roles = prisma
        .role()
        .find_many(vec![])
        .exec()
        .await?
        .into_iter()
        .map(|n| n.id)
        .collect::<HashSet<_>>();

    // for every role in the database
    for id_i64 in prisma_roles {
        let id = serenity::RoleId::new(id_i64 as u64);

        if let Some(role) = roles.get(&id) {
            super::role::update(role.clone()).await?;
        } else {
            super::role::delete(id).await?;
        }
    }

    // for every role in the guild
    for (id, role) in roles {
        if !prisma_roles.contains(&id.into()) {
            super::role::create(role).await?;
        }
    }

    Ok(())
}

async fn members() -> Result<(), MemberLogError> {
    let prisma = prisma::create().await?;

    let members: HashMap<serenity::UserId, serenity::Member> = todo!("Comms: get members");

    let prisma_members = prisma
        .user()
        .find_many(vec![])
        .with(user::roles::fetch(vec![]))
        .exec()
        .await?
        .into_iter()
        .map(|n| n.id)
        .collect::<HashSet<_>>();

    for (id, member) in members {
        if prisma_members.contains(&id.into()) {
            super::member::update(member).await?;
        } else {
            super::member::join(member).await?;
        }
    }

    for id_i64 in prisma_members {
        let id = serenity::UserId::new(id_i64 as u64);

        if !members.contains_key(&id) {
            super::member::leave(id).await?;
        }
    }

    Ok(())
}

async fn categories() -> Result<(), CategoryLogError> {
    let prisma = prisma::create().await?;

    let categories: HashMap<serenity::ChannelId, serenity::GuildChannel> =
        todo!("Comms: get categories");

    let prisma_categories = prisma
        .channel_category()
        .find_many(vec![])
        .exec()
        .await?
        .into_iter()
        .map(|n| n.id)
        .collect::<HashSet<_>>();

    for (id, category) in categories {
        if prisma_categories.contains(&id.into()) {
            super::category::update(category).await?;
        } else {
            super::category::create(category).await?;
        }
    }

    for id_i64 in prisma_categories {
        let id = serenity::ChannelId::new(id_i64 as u64);

        if !categories.contains_key(&id) {
            super::category::delete(id).await?;
        }
    }

    Ok(())
}

async fn channels() -> Result<(), ChannelLogError> {
    let prisma = prisma::create().await?;

    let channels: HashMap<serenity::ChannelId, serenity::GuildChannel> =
        todo!("Comms: get channels");

    let prisma_channels = prisma
        .channel()
        .find_many(vec![])
        .exec()
        .await?
        .into_iter()
        .map(|n| n.id)
        .collect::<HashSet<_>>();

    for (id, channel) in channels {
        if prisma_channels.contains(&id.into()) {
            super::channel::update(channel).await?;
        } else {
            super::channel::create(channel).await?;
        }
    }

    for id_i64 in prisma_channels {
        let id = serenity::ChannelId::new(id_i64 as u64);

        if !channels.contains_key(&id) {
            super::channel::delete(id).await?;
        }
    }

    Ok(())
}

pub async fn ready() -> Result<(), ReadyEventError> {
    roles().await?;
    members().await?;
    categories().await?;
    channels().await?;

    Ok(())
}
