use crate::prelude::*;

pub async fn create(channel: serenity::GuildChannel) -> Result<(), ChannelLogError> {
    let prisma = prisma::create().await?;

    // to hell with all threads
    if channel.parent_id.is_some() {
        bail!(ChannelLogError::Thread(channel.id));
    }

    let mut params = vec![];

    if let Some(topic) = channel.topic.as_ref() {
        params.push(channel::topic::set(Some(topic.to_string())))
    }

    if let Some(category_id) = channel.parent_id {
        params.push(channel::category::connect(channel_category::id::equals(
            category_id,
        )));
    }

    prisma
        .channel()
        .create(
            channel.id.database_id(),
            channel.name,
            channel.nsfw,
            match channel.kind {
                serenity::ChannelType::News => ChannelType::News,
                serenity::ChannelType::Text => ChannelType::Text,
                serenity::ChannelType::Stage => ChannelType::Stage,
                serenity::ChannelType::Voice => ChannelType::Voice,
                _ => return Ok(()),
            },
            params,
        )
        .exec()
        .await?;

    Ok(())
}

pub async fn update(channel: serenity::GuildChannel) -> Result<(), ChannelLogError> {
    if channel.thread_metadata.is_some() {
        bail!(ChannelLogError::Thread(channel.id));
    }

    let prisma = prisma::create().await?;

    prisma
        .channel()
        .update(
            channel::id::equals(channel.id.database_id()),
            vec![
                channel::name::set(&channel.name),
                channel::topic::set(channel.topic),
                channel::nsfw::set(channel.nsfw),
                if let Some(id) = channel.parent_id {
                    channel::category::connect(channel_category::id::equals(id))
                } else {
                    channel::category::disconnect()
                },
            ],
        )
        .exec()
        .await?;

    Ok(())
}

pub async fn delete(channel_id: serenity::ChannelId) -> Result<(), ChannelLogError> {
    let prisma = prisma::create().await?;

    prisma
        .channel()
        .update(
            channel::id::equals(channel_id),
            vec![channel::deleted::set(true)],
        )
        .exec()
        .await?;

    prisma
        .message()
        .update_many(
            vec![message::channel_id::equals(channel_id)],
            vec![message::deleted::set(true)],
        )
        .exec()
        .await?;

    // delete attachments
    let messages = prisma
        .message()
        .find_many(vec![message::channel_id::equals(channel_id)])
        .exec()
        .await?;

    let updates = messages
        .iter()
        .map(|n| n.id.clone())
        .map(|id| {
            prisma.attachment().update_many(
                vec![attachment::message_id::equals(id)],
                vec![attachment::deleted::set(true)],
            )
        })
        .collect_vec();

    prisma._batch(updates).await?;

    Ok(())
}

pub async fn read(
    channel_id: serenity::ChannelId,
) -> Result<prisma::data::ChannelData, ChannelLogError> {
    let prisma = prisma::create().await?;

    let channel = prisma
        .channel()
        .find_unique(channel::id::equals(channel_id))
        .with(channel::category::fetch())
        .with(channel::messages::fetch(vec![]))
        .exec()
        .await?
        .make_error(ChannelLogError::NotFound(channel_id))?;

    Ok(channel)
}
