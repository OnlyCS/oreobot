mod category;
mod channel;
mod interaction;
mod message;

use crate::prelude::*;

// macro_rules! do_register {
//     ($emitter:expr, $event:expr, |$($arg:ident:$typ:ty),*|, $fn:expr, $($fnarg:expr),*) => {
//         $emitter.on_async(&$event, move |$($arg:$typ,)* ctx: serenity::Context| {
//             $fn($($fnarg)*, ctx)
//         });
//     };
// }

pub async fn register(ctx: &serenity::Context) -> Result<()> {
    let emitter_mutex = Arc::clone(
        ctx.data
            .read()
            .await
            .get::<EventEmitterTypeKey>()
            .context("Could not find event emitter")?,
    );

    let mut emitter = emitter_mutex.lock().await;

    // do_register!(emitter, EmitterEvent::AnyInteraction, |payload: AnyInteractionPayload|, interaction::create, payload.0);

    emitter.on_async(
        &EmitterEvent::AnyInteraction,
        |payload: AnyInteractionPayload, ctx: serenity::Context| {
            interaction::create(payload.0, ctx)
        },
    );

    emitter.on_async(
        &EmitterEvent::CategoryCreate,
        |payload: CategoryCreatePayload, ctx: serenity::Context| category::create(payload.0, ctx),
    );

    emitter.on_async(
        &EmitterEvent::AnyCategoryUpdate,
        |payload: CategoryUpdatePayload, ctx: serenity::Context| category::update(payload.0, ctx),
    );

    emitter.on_async(
        &EmitterEvent::AnyCategoryDelete,
        |payload: CategoryDeletePayload, ctx: serenity::Context| {
            category::delete(payload.0.id, ctx)
        },
    );

    emitter.on_async(
        &EmitterEvent::ChannelCreate,
        |payload: ChannelCreatePayload, ctx: serenity::Context| channel::create(payload.0, ctx),
    );

    emitter.on_async(
        &EmitterEvent::AnyChannelUpdate,
        |payload: ChannelUpdatePayload, ctx: serenity::Context| channel::update(payload.0, ctx),
    );

    emitter.on_async(
        &EmitterEvent::AnyChannelDelete,
        |payload: ChannelDeletePayload, ctx: serenity::Context| channel::delete(payload.0.id, ctx),
    );

    emitter.on_async(
        &EmitterEvent::MessageCreate,
        |payload: MessageCreatePayload, ctx: serenity::Context| message::create(payload.0, ctx),
    );

    emitter.on_async(
        &EmitterEvent::AnyMessageUpdate,
        |payload: MessageUpdatePayload, ctx: serenity::Context| message::update(payload.0, ctx),
    );

    emitter.on_async(
        &EmitterEvent::AnyMessageDelete,
        |payload: MessageDeletePayload, ctx: serenity::Context| {
            message::delete(payload.message_id, ctx)
        },
    );

    Ok(())
}
