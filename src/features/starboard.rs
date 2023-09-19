use crate::prelude::*;

macro_rules! star {
    ($message:expr, $seren:expr) => {{
        let prisma = prisma::create().await?;
        let message = $message;

        let message_data = prisma
            .message()
            .find_unique(message::id::equals(message.id.to_string()))
            .with(message::pin::fetch())
            .exec()
            .await?
            .make_error(StarboardError::MessageNotInDatabase(message.id))?;

        let existing_message_pin = message_data.pin()?;
        let is_pinned = existing_message_pin.map(|n| !n.removed).unwrap_or(false);

        if is_pinned {
            return Ok(());
        }

        let cloned = clone::clone(clone::CloneArgsBuilder::build_from(move |args| {
            args.message(message.clone());
            args.destination(nci::channels::STARRED);
            args.ctx($seren);
        })?)
        .await?;

        if let Some(pin) = existing_message_pin {
            prisma
                .message_pin()
                .update(
                    message_pin::pinned_message_id::equals(pin.pinned_message_id.clone()),
                    vec![
                        message_pin::removed::set(false),
                        message_pin::removed_reason::set(None),
                        message_pin::original::connect(message::id::equals(message.id.to_string())),
                    ],
                )
                .exec()
                .await?;
        } else {
            prisma
                .message_pin()
                .create(
                    cloned.id.to_string(),
                    message::id::equals(message.id.to_string()),
                    vec![],
                )
                .exec()
                .await?;
        }

        Ok(())
    } as Result<(), StarboardError>};
}

pub async fn star_no_interaction(
    ctx: &serenity::Context,
    message: &serenity::Message,
) -> Result<(), StarboardError> {
    star!(message, ctx.clone())?;

    Ok(())
}

// plans to right-click star interaction
pub async fn star_interaction(
    ctx: &Context<'_>,
    message: &serenity::Message,
) -> Result<(), StarboardError> {
    star!(message, ctx.serenity_context().clone())?;

    Ok(())
}

pub async fn register(ctx: &serenity::Context) {
    let data_arc = data::get_serenity(ctx).await;
    let mut data = data_arc.lock().await;
    let emitter = &mut data.emitter;

    emitter.on_filter(
        events::MessageReactionAdd,
        |payload, ctx| async move {
            star_no_interaction(&ctx, &payload.message).await?;
            Ok(())
        },
        |payload| {
            if let serenity::ReactionType::Unicode(ucode) = payload.reaction.emoji {
                ucode == emoji::PIN_EMOJI
            } else {
                false
            }
        },
    );
}
