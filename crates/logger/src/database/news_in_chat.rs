use crate::database::message as message_log;
use crate::prelude::*;
use std::collections::HashMap;

pub async fn create(
    news: serenity::Message,
    chat: serenity::MessageId,
) -> Result<(), NewsInChatLogError> {
    let news_id = message_log::create_unchecked(news).await?;
    let prisma = prisma::create().await?;

    prisma
        .news_in_chat()
        .create(message::id::equals(news_id), chat, vec![])
        .exec()
        .await?;

    Ok(())
}

pub async fn read(source: serenity::MessageId) -> Result<serenity::MessageId, NewsInChatLogError> {
    let prisma = prisma::create().await?;

    let data = prisma
        .news_in_chat()
        .find_unique(news_in_chat::source_id::equals(source))
        .with(news_in_chat::source::fetch())
        .exec()
        .await?
        .make_error(NewsInChatLogError::NotFound(source))?;

    Ok(serenity::MessageId::new(data.clone as u64))
}

pub async fn all() -> Result<HashMap<serenity::MessageId, serenity::MessageId>, NewsInChatLogError>
{
    let prisma = prisma::create().await?;

    let news_in_chat = prisma
        .news_in_chat()
        .find_many(vec![])
        .with(news_in_chat::source::fetch())
        .exec()
        .await?
        .into_iter()
        .map(|n| (n.source_id, n.clone))
        .map(|(source, clone)| {
            (
                serenity::MessageId::from(source as u64),
                serenity::MessageId::from(clone as u64),
            )
        })
        .collect();

    Ok(news_in_chat)
}
