use crate::prelude::*;

pub async fn create(
    source: serenity::MessageId,
    clone: serenity::MessageId,
    destination: serenity::ChannelId,
    reason: MessageCloneReason,
    update: bool,
    update_delete: bool,
) -> Result<(), MessageCloneLogError> {
    let prisma = prisma::create().await?;

    prisma
        .message_clone()
        .create(
            clone,
            message::id::equals(source),
            update,
            update_delete,
            channel::id::equals(destination),
            reason,
            vec![],
        )
        .exec()
        .await?;

    Ok(())
}

pub async fn read(
    source: serenity::MessageId,
) -> Result<HashMap<serenity::MessageId, prisma::data::MessageCloneData>, MessageCloneLogError> {
    let prisma = prisma::create().await?;

    let message_clones = prisma
        .message_clone()
        .find_many(vec![message_clone::source_id::equals(source)])
        .with(message_clone::source::fetch())
        .with(message_clone::destination::fetch())
        .exec()
        .await?
        .into_iter()
        .fold(HashMap::new(), |mut collect, item| {
            collect.insert(serenity::MessageId::new(item.id as u64), item);
            collect
        });

    Ok(message_clones)
}
