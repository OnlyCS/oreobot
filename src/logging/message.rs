use crate::prelude::*;

pub async fn create(
    ctx: serenity::Context,
    message: serenity::Message,
) -> Result<(), LoggingError> {
    if message.webhook_id.is_some() {
        bail!(LoggingError::WebhookMessage(message.id));
    }

    if message.channel_id == nci::channels::NEWS {
        bail!(LoggingError::NewsMessage(message.id));
    }

    let data_arc = data::get_serenity(&ctx).await;
    let mut data = data_arc.lock().await;
    let cache = &mut data.cache;

    if cache
        .get::<cache::impersonate::Impersonation>()
        .await?
        .get(&message.author.id)
        .cloned()
        .flatten()
        .is_some()
    {
        bail!(LoggingError::UserImpersonated(message.author.id));
    }

    let prisma = prisma::create().await?;

    prisma
        .message()
        .create(
            message.id.to_string(),
            message.content,
            user::id::equals(message.author.id.to_string()),
            channel::id::equals(message.channel_id.to_string()),
            vec![],
        )
        .exec()
        .await?;

    let attachments = message
        .attachments
        .iter()
        .map(|attachment| {
            (
                attachment.id.to_string(),
                attachment.filename.clone(),
                attachment.url.clone(),
                attachment.size as i64, //cannot exceed 100gb, so i64 is fine
                message.id.to_string(),
                vec![],
            )
        })
        .collect_vec();

    prisma
        .attachment()
        .create_many(attachments)
        .exec()
        .await
        .unwrap();

    Ok(())
}

pub async fn update(message: serenity::MessageUpdateEvent) -> Result<(), LoggingError> {
    let prisma = prisma::create().await?;

    prisma
        .message()
        .update(
            message::id::equals(message.id.to_string()),
            vec![
                message::content::set(message.content.unwrap_or("".to_string())),
                message::edited::set(true),
            ],
        )
        .exec()
        .await?;

    Ok(())
}

pub async fn delete(message_id: serenity::MessageId) -> Result<(), LoggingError> {
    let prisma = prisma::create().await?;

    // dont delete impersonated messages yet
    let message_data = prisma
        .message()
        .find_unique(message::id::equals(message_id.to_string()))
        .with(message::impersonated_message::fetch())
        .exec()
        .await?
        .make_error(LoggingError::NotFound(format!(
            "message with id {}",
            message_id
        )))?;

    if message_data.impersonated_message.flatten().is_some() {
        return Ok(());
    }

    prisma
        .message()
        .update(
            message::id::equals(message_id.to_string()),
            vec![message::deleted::set(true)],
        )
        .exec()
        .await?;

    prisma
        .attachment()
        .update_many(
            vec![attachment::message_id::equals(message_id.to_string())],
            vec![attachment::deleted::set(true)],
        )
        .exec()
        .await?;

    Ok(())
}
