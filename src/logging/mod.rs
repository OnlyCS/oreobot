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
    emitter.on_async(events::CommandInteractionEvent, |interaction, ctx| {
        interaction::command(interaction, ctx)
    });

    emitter.on_async(events::ComponentInteractionEvent, |interaction, ctx| {
        interaction::message_component(interaction, ctx)
    });

    emitter.on_async(events::ModalInteractionEvent, |interaction, ctx| {
        interaction::modal_submit(interaction, ctx)
    });

    // category events
    emitter.on_async(events::CategoryCreateEvent, |category, ctx| {
        category::create(category, ctx)
    });

    emitter.on_async(events::CategoryUpdateEvent, |category, ctx| {
        category::update(category, ctx)
    });

    emitter.on_async(events::CategoryDeleteEvent, |category, ctx| {
        category::delete(category.id, ctx)
    });

    // channel events
    emitter.on_async(events::ChannelCreateEvent, |channel, ctx| {
        channel::create(channel, ctx)
    });

    emitter.on_async(events::ChannelUpdateEvent, |channel, ctx| {
        channel::update(channel, ctx)
    });

    emitter.on_async(events::ChannelDeleteEvent, |channel, ctx| {
        channel::delete(channel.id, ctx)
    });

    // message events
    emitter.on_async(events::MessageCreateEvent, |message, ctx| {
        message::create(message, ctx)
    });

    emitter.on_async(events::MessageUpdateEvent, |event, ctx| {
        message::update(event, ctx)
    });

    emitter.on_async(events::MessageDeleteEvent, |payload, ctx| {
        message::delete(payload.message_id, ctx)
    });

    // role events
    emitter.on_async(events::RoleCreateEvent, |role, ctx| role::create(role, ctx));
    emitter.on_async(events::RoleUpdateEvent, |role, ctx| role::update(role, ctx));
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
