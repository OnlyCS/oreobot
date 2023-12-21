use crate::prelude::*;

pub async fn perform(ctx: serenity::Context, message: Message) -> Result<(), NewsCloneError> {
    bail_assert_eq!(
        message.channel_id,
        nci::channels::NEWS,
        NewsCloneError::IncorrectChannel
    );

    // messages without @everyone or @here should be deleted and copied to chat instead
    // starts_with(_) is for in case of testing and stuff
    let eligible = message.content.contains("@everyone")
        || message.content.contains("@here")
        || message.content.starts_with("_");

    clone::message_clone(
        &ctx,
        &message,
        nci::channels::CHAT,
        clone::MessageCloneOptions {
            reason: MessageCloneReason::NewsInChat,
            delete: eligible,
            ..Default::default()
        },
    )
    .await?;

    if !eligible {
        message.delete(&ctx).await?;
    }

    Ok(())
}

// if we aren't integrating with smarty, we must handle this ourselves
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
