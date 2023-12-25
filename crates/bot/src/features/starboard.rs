use crate::prelude::*;

const PIN_EMOJI: &str = "📌";

pub async fn perform(ctx: &serenity::Context, message: Message) -> Result<(), StarboardError> {
    let mut logger = Client::<LoggingServer>::new().await?;

    let LoggingResponse::MessageClonesOk(clones) = logger
        .send(LoggingRequest::MessageCloneRead { source: message.id })
        .await?
    else {
        bail!(RouterError::<LoggingServer>::InvalidResponse);
    };

    if clones
        .into_values()
        .filter(|clone| clone.reason == MessageCloneReason::Starboard)
        .next()
        .is_some()
    {
        bail!(StarboardError::DoubleStar)
    }

    clone::message_clone(
        ctx,
        &message,
        nci::channels::STARRED,
        MessageCloneOptions {
            update: false,
            delete: false,
            reason: MessageCloneReason::Starboard,
            ..Default::default()
        },
    )
    .await?;

    Ok(())
}

pub async fn register() {
    mpmc::on(|ctx, event, _| async move {
        let FullEvent::ReactionAdd {
            add_reaction: reaction,
        } = event
        else {
            bail!(EventError::UnwantedEvent)
        };

        if !match reaction.emoji {
            serenity::ReactionType::Unicode(ref emoji) => emoji == PIN_EMOJI,
            _ => false,
        } {
            bail!(EventError::UnwantedEvent)
        }

        perform(&ctx, reaction.message(&ctx).await?).await?;

        Ok(())
    })
    .await;

    mpmc::on(|ctx, event, _| async move {
        let FullEvent::ChannelPinsUpdate { pin: event } = event else {
            bail!(EventError::UnwantedEvent)
        };

        let channel = event.channel_id;

        let pin_results = future::join_all(
            ctx.http
                .get_pins(channel)
                .await?
                .into_iter()
                .map(|i| future::join(channel.unpin(&ctx, i.id), perform(&ctx, i))),
        )
        .await;

        for res in pin_results {
            let (unpin, clone) = res;

            unpin?;
            clone?;
        }

        Ok(())
    })
    .await;
}
