use crate::prelude::*;

pub async fn create(channel: serenity::GuildChannel) -> Result<(), LoggingError> {
    let prisma = prisma::create().await?;

    // to hell with all threads
    if channel.is_thread() {
        return Err(LoggingError::ChannelIsThread(channel.id));
    }

    let mut params = vec![];

    if let Some(topic) = channel.topic.as_ref() {
        params.push(channel::topic::set(Some(topic.to_string())))
    }

    if let Some(category_id) = channel.parent_id {
        params.push(channel::category::connect(channel_category::id::equals(
            category_id.to_string(),
        )));
    }

    prisma
        .channel()
        .create(
            channel.id.to_string(),
            channel.name.clone(),
            channel.is_nsfw(),
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

pub async fn update(channel: serenity::GuildChannel) -> Result<(), LoggingError> {
    if channel.is_thread() {
        return Err(LoggingError::ChannelIsThread(channel.id));
    }

    let prisma = prisma::create().await?;

    prisma
        .channel()
        .update(
            channel::id::equals(channel.id.to_string()),
            vec![
                channel::name::set(channel.name.clone()),
                channel::topic::set(channel.topic.clone()),
                channel::nsfw::set(channel.is_nsfw()),
                if let Some(id) = channel.parent_id {
                    channel::category::connect(channel_category::id::equals(id.to_string()))
                } else {
                    channel::category::disconnect()
                },
            ],
        )
        .exec()
        .await?;

    Ok(())
}

pub async fn delete(channel: serenity::ChannelId) -> Result<(), LoggingError> {
    let prisma = prisma::create().await?;

    prisma
        .channel()
        .update(
            channel::id::equals(channel.to_string()),
            vec![channel::deleted::set(true)],
        )
        .exec()
        .await?;

    prisma
        .message()
        .update_many(
            vec![message::channel_id::equals(channel.to_string())],
            vec![message::deleted::set(true)],
        )
        .exec()
        .await?;

    // delete attachments
    let messages = prisma
        .message()
        .find_many(vec![message::channel_id::equals(channel.to_string())])
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
