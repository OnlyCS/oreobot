mod category;
mod channel;
mod interaction;
mod member;
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

    // interaction events
    emitter.on_async(events::CommandInteractionEvent, |interaction, _| {
        interaction::command(interaction)
    });

    emitter.on_async(events::ComponentInteractionEvent, |interaction, _| {
        interaction::message_component(interaction)
    });

    emitter.on_async(events::ModalInteractionEvent, |interaction, _| {
        interaction::modal_submit(interaction)
    });

    // category events
    emitter.on_async(events::CategoryCreateEvent, |category, _| {
        category::create(category)
    });

    emitter.on_async(events::CategoryUpdateEvent, |category, _| {
        category::update(category)
    });

    emitter.on_async(events::CategoryDeleteEvent, |category, _| {
        category::delete(category.id)
    });

    // channel events
    emitter.on_async(events::ChannelCreateEvent, |channel, _| {
        channel::create(channel)
    });

    emitter.on_async(events::ChannelUpdateEvent, |channel, _| {
        channel::update(channel)
    });

    emitter.on_async(events::ChannelDeleteEvent, |channel, _| {
        channel::delete(channel.id)
    });

    // message events
    emitter.on_async(events::MessageCreateEvent, |message, _| {
        message::create(message)
    });

    emitter.on_async(events::MessageUpdateEvent, |event, _| {
        message::update(event)
    });

    emitter.on_async(events::MessageDeleteEvent, |payload, _| {
        message::delete(payload.message_id)
    });

    // role events
    emitter.on_async(events::RoleCreateEvent, |role, _| role::create(role));
    emitter.on_async(events::RoleUpdateEvent, |role, _| role::update(role));
    emitter.on_async(events::RoleDeleteEvent, |payload, ctx| {
        role::delete(payload.role_id, ctx)
    });

    // member events
    emitter.on_async(events::MemberJoinEvent, |member, ctx| {
        member::join(member, ctx)
    });

    emitter.on_async(events::MemberUpdateEvent, |member, ctx| {
        member::update(member, ctx)
    });

    emitter.on_async(events::MemberLeaveEvent, |payload, ctx| {
        member::leave(payload.user.id, ctx)
    });

    // ready event
    emitter.on_async(events::BotReadyEvent, |_, ctx| async move {
        ready::on_ready(ctx.clone()).await?;

        ctx.set_presence(
            Some(serenity::Activity::playing("with BOMBS")),
            serenity::OnlineStatus::Online,
        )
        .await;

        Ok(())
    });

    Ok(())
}
