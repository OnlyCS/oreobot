use crate::prelude::*;

pub(in crate::database) async fn create_unchecked(
    message: serenity::Message,
) -> Result<serenity::MessageId, MessageLogError> {
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

    Ok(message.id)
}

pub async fn create(message: serenity::Message) -> Result<(), MessageLogError> {
    if message.webhook_id.is_some() {
        bail!(MessageLogError::WebhookMessage(message.id));
    }

    if message.channel_id == nci::channels::NEWS {
        bail!(MessageLogError::NewsMessage(message.id));
    }

    create_unchecked(message).await?;

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
        .with(message::clones::fetch(vec![]))
        .exec()
        .await?
        .make_error(MessageLogError::NotFound(message_id))?;

    if message
        .clones()?
        .into_iter()
        .any(|clone| clone.reason == MessageCloneReason::Impersonation)
    {
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
        .with(message::clones::fetch(vec![]))
        .with(message::author::fetch())
        .exec()
        .await?
        .make_error(MessageLogError::NotFound(message_id))?;

    Ok(message)
}
