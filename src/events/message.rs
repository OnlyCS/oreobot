use std::collections::HashSet;

use crate::prelude::*;

pub async fn create(message: serenity::Message, prisma_client: &PrismaClient) -> Result<()> {
    prisma_client
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

    for attachment in message.attachments {
        prisma_client
            .attachment()
            .create(
                attachment.id.to_string(),
                attachment.filename,
                attachment.url,
                attachment.size as i64, //cannot exceed 100gb, so i64 is fine
                message::id::equals(message.id.to_string()),
                vec![],
            )
            .exec()
            .await?;
    }

    Ok(())
}

pub async fn update(
    message: serenity::MessageUpdateEvent,
    prisma_client: &PrismaClient,
) -> Result<()> {
    let prisma_message = prisma_client
        .message()
        .update(
            message::id::equals(message.id.to_string()),
            vec![
                message::content::set(message.content.unwrap_or("".to_string())),
                message::edited::set(true),
            ],
        )
        .with(message::attachments::fetch(vec![
            attachment::message_id::equals(message.id.to_string()),
        ]))
        .exec()
        .await?;

    if let Some(attachments) = message.attachments {
        let attachment_hs: HashSet<String> = attachments.iter().map(|a| a.id.to_string()).collect();

        let new_attachment_hs: HashSet<String> = prisma_message
            .attachments
            .unwrap_or(vec![])
            .iter()
            .map(|a| a.id.to_string())
            .collect();

        let diff = &new_attachment_hs - &attachment_hs;

        let mut where_params = vec![];

        for id in diff.iter().cloned() {
            where_params.push(attachment::id::equals(id));
        }

        prisma_client
            .attachment()
            .update_many(where_params, vec![attachment::deleted::set(true)])
            .exec()
            .await?;
    }

    Ok(())
}

pub async fn delete(message_id: serenity::MessageId, prisma_client: &PrismaClient) -> Result<()> {
    prisma_client
        .message()
        .update(
            message::id::equals(message_id.to_string()),
            vec![message::deleted::set(true)],
        )
        .exec()
        .await?;

    prisma_client
        .attachment()
        .update_many(
            vec![attachment::message_id::equals(message_id.to_string())],
            vec![attachment::deleted::set(true)],
        )
        .exec()
        .await?;

    Ok(())
}
