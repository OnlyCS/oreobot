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
