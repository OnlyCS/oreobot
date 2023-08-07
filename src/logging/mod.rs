mod category;
mod channel;
mod interaction;
mod message;
mod role;

pub mod ready;

use crate::prelude::*;

pub async fn register(ctx: &serenity::Context) -> Result<()> {
    let emitter_mutex = Arc::clone(
        ctx.data
            .read()
            .await
            .get::<EventEmitterTypeKey>()
            .context("Could not find event emitter")?,
    );

    let mut emitter = emitter_mutex.lock().await;

    emitter.on_async(event::CommandInteractionEvent, |interaction, ctx| {
        interaction::command(interaction, ctx)
    });

    emitter.on_async(event::ComponentInteractionEvent, |interaction, ctx| {
        interaction::message_component(interaction, ctx)
    });

    emitter.on_async(event::ModalInteractionEvent, |interaction, ctx| {
        interaction::modal_submit(interaction, ctx)
    });

    emitter.on_async(event::CategoryCreateEvent, |category, ctx| {
        category::create(category, ctx)
    });

    emitter.on_async(event::CategoryUpdateEvent, |category, ctx| {
        category::update(category, ctx)
    });

    emitter.on_async(event::CategoryDeleteEvent, |category, ctx| {
        category::delete(category.id, ctx)
    });

    emitter.on_async(event::ChannelCreateEvent, |channel, ctx| {
        channel::create(channel, ctx)
    });

    emitter.on_async(event::ChannelUpdateEvent, |channel, ctx| {
        channel::update(channel, ctx)
    });

    emitter.on_async(event::ChannelDeleteEvent, |channel, ctx| {
        channel::delete(channel.id, ctx)
    });

    emitter.on_async(event::MessageCreateEvent, |message, ctx| {
        message::create(message, ctx)
    });

    emitter.on_async(event::MessageUpdateEvent, |event, ctx| {
        message::update(event, ctx)
    });

    emitter.on_async(event::MessageDeleteEvent, |payload, ctx| {
        message::delete(payload.message_id, ctx)
    });

    emitter.on_async(event::RoleUpdateEvent, |role, ctx| role::update(role, ctx));

    emitter.on_async(event::RoleDeleteEvent, |payload, ctx| {
        role::delete(payload.role_id, ctx)
    });

    emitter.on_async(event::BotReadyEvent, |_, ctx| async move {
        ready::ready(ctx.clone()).await?;

        ctx.set_presence(
            Some(serenity::Activity::playing("with BOMBS")),
            serenity::OnlineStatus::Online,
        )
        .await;

        Ok(())
    });

    Ok(())
}
