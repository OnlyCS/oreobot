use crate::prelude::*;

pub async fn create(message: serenity::Message) -> Result<(), MessageLogError> {
    if message.webhook_id.is_some() {
        bail!(MessageLogError::WebhookMessage(message.id));
    }

    if message.channel_id == nci::channels::NEWS {
        bail!(MessageLogError::NewsMessage(message.id));
    }

    let prisma = prisma::create().await?;

    prisma
        .message()
        .create(
            message.id,
            message.content,
            user::id::equals(message.author.id),
            channel::id::equals(message.channel_id),
            vec![],
        )
        .exec()
        .await?;

    let attachments = message
        .attachments
        .iter()
        .map(|attachment| {
            prisma.attachment().create(
                attachment.id,
                &attachment.filename,
                &attachment.url,
                attachment.size,
                message::id::equals(message.id),
                vec![],
            )
        })
        .collect_vec();

    prisma._batch(attachments).await?;

    Ok(())
}

pub async fn update(message: serenity::MessageUpdateEvent) -> Result<(), MessageLogError> {
    let prisma = prisma::create().await?;

    prisma
        .message()
        .update(
            message::id::equals(message.id),
            vec![
                message::content::set(message.content.unwrap_or("".to_string())),
                message::edited::set(true),
            ],
        )
        .exec()
        .await?;

    Ok(())
}

pub async fn delete(message_id: serenity::MessageId) -> Result<(), MessageLogError> {
    let prisma = prisma::create().await?;

    let message = prisma
        .message()
        .find_unique(message::id::equals(message_id))
        .with(message::impersonated_message::fetch())
        .exec()
        .await?
        .make_error(MessageLogError::NotFound(message_id))?;

    if message.impersonated_message.flatten().is_some() {
        bail!(MessageLogError::MessageImpersonated(message_id));
    }

    prisma
        .message()
        .update(
            message::id::equals(message_id),
            vec![message::deleted::set(true)],
        )
        .exec()
        .await?;

    prisma
        .attachment()
        .update_many(
            vec![attachment::message_id::equals(message_id)],
            vec![attachment::deleted::set(true)],
        )
        .exec()
        .await?;

    Ok(())
}

pub async fn read(
    message_id: serenity::MessageId,
) -> Result<prisma::data::MessageData, MessageLogError> {
    let prisma = prisma::create().await?;

    let message = prisma
        .message()
        .find_unique(message::id::equals(message_id))
        .with(message::attachments::fetch(vec![]))
        .with(message::channel::fetch())
        .with(message::pin::fetch())
        .with(message::impersonated_message::fetch())
        .with(message::chat_message::fetch())
        .with(message::author::fetch())
        .exec()
        .await?
        .make_error(MessageLogError::NotFound(message_id))?;

    Ok(message)
}
