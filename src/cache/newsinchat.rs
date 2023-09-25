use crate::prelude::*;

pub struct NewsInChat;

#[async_trait]
impl cache::CacheItem for NewsInChat {
    type Value = HashMap<serenity::MessageId, serenity::MessageId>;
    type UpdateValue = serenity::MessageId;

    type InnerKey = serenity::Message;
    type Get = Option<serenity::MessageId>;

    async fn default_value() -> Result<Self::Value, AnyError> {
        let prisma = prisma::create().await?;

        Ok(prisma
            .news_in_chat()
            .find_many(vec![])
            .exec()
            .await?
            .into_iter()
            .map(|x| (x.source_id, x.fake_message_id))
            .map(|(source, clone)| {
                (
                    serenity::MessageId(u64::from_str(&source).unwrap()),
                    serenity::MessageId(u64::from_str(&clone).unwrap()),
                )
            })
            .collect::<HashMap<_, _>>())
    }

    async fn get(
        ctx: &serenity::Context,
        key: Self::InnerKey,
        value: Self::Value,
    ) -> Result<Self::Get, AnyError> {
        Ok(value.get(&key.id).copied())
    }

    async fn update(
        _ctx: &serenity::Context,
        current_value: &mut Self::Value,
        key: Self::InnerKey,
        value: Self::UpdateValue,
    ) -> Result<(), AnyError> {
        let source = key;
        let clone = value;
        current_value.insert(source.id, clone);

        let prisma = prisma::create().await?;

        prisma
            .message()
            .create(
                source.id.to_string(),
                source.content,
                user::id::equals(source.author.id.to_string()),
                channel::id::equals(source.channel_id.to_string()),
                vec![],
            )
            .exec()
            .await?;

        let attachments = source
            .attachments
            .iter()
            .map(|attachment| {
                (
                    attachment.id.to_string(),
                    attachment.filename.clone(),
                    attachment.url.clone(),
                    attachment.size as i64, //cannot exceed 100gb, so i64 is fine
                    source.id.to_string(),
                    vec![],
                )
            })
            .collect_vec();

        prisma.attachment().create_many(attachments).exec().await?;

        prisma
            .news_in_chat()
            .create(
                message::id::equals(source.id.to_string()),
                clone.to_string(),
                vec![],
            )
            .exec()
            .await?;

        Ok(())
    }
}
