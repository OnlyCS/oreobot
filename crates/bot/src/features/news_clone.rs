use crate::prelude::*;

pub async fn perform(ctx: serenity::Context, message: Message) -> Result<(), NewsCloneError> {
    bail_assert_eq!(
        message.channel_id,
        nci::channels::NEWS,
        NewsCloneError::IncorrectChannel
    );

    clone::message_clone(
        &ctx,
        &message,
        nci::channels::CHAT,
        clone::MessageCloneOptions {
            reason: MessageCloneReason::NewsInChat,
            ..Default::default()
        },
    )
    .await?;

    Ok(())
}

#[cfg(not(feature = "smarty-integration"))]
pub async fn register() {
    mpmc::on(|ctx, event, _| async move {
        let FullEvent::Message {
            new_message: message,
        } = event
        else {
            bail!(EventError::UnwantedEvent)
        };

        if message.channel_id != nci::channels::NEWS {
            bail!(EventError::UnwantedEvent)
        }

        perform(ctx, message).await?;

        Ok(())
    })
    .await;
}
