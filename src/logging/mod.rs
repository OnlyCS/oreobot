mod category;
mod channel;
mod interaction;
mod member;
mod message;
mod role;

pub mod ready;

use crate::prelude::*;

pub async fn register(ctx: &serenity::Context) -> Result<()> {
    let data_arc = data::get_serenity(ctx).await?;
    let mut data = data_arc.lock().await;
    let emitter = &mut data.emitter;

    // interaction events
    emitter.on(events::CommandInteractionEvent, |interaction, _| {
        interaction::command(interaction)
    });

    emitter.on(events::ComponentInteractionEvent, |interaction, _| {
        interaction::message_component(interaction)
    });

    emitter.on(events::ModalInteractionEvent, |interaction, _| {
        interaction::modal_submit(interaction)
    });

    // category events
    emitter.on(events::CategoryCreateEvent, |category, _| {
        category::create(category)
    });

    emitter.on(events::CategoryUpdateEvent, |category, _| {
        category::update(category)
    });

    emitter.on(events::CategoryDeleteEvent, |category, _| {
        category::delete(category.id)
    });

    // channel events
    emitter.on(events::ChannelCreateEvent, |channel, _| {
        channel::create(channel)
    });

    emitter.on(events::ChannelUpdateEvent, |channel, _| {
        channel::update(channel)
    });

    emitter.on(events::ChannelDeleteEvent, |channel, _| {
        channel::delete(channel.id)
    });

    // message events
    emitter.on(events::MessageCreateEvent, |message, _| {
        message::create(message)
    });

    emitter.on(events::MessageUpdateEvent, |event, _| {
        message::update(event)
    });

    emitter.on(events::MessageDeleteEvent, |payload, _| {
        message::delete(payload.message_id)
    });

    // role events
    emitter.on(events::RoleCreateEvent, |role, _| role::create(role));
    emitter.on(events::RoleUpdateEvent, |role, _| role::update(role));
    emitter.on(events::RoleDeleteEvent, |payload, ctx| {
        role::delete(payload.role_id, ctx)
    });

    // member events
    emitter.on(events::MemberJoinEvent, |member, ctx| {
        member::join(member, ctx)
    });

    emitter.on(events::MemberUpdateEvent, |member, ctx| {
        member::update(member, ctx)
    });

    emitter.on(events::MemberLeaveEvent, |payload, ctx| {
        member::leave(payload.user.id, ctx)
    });

    // ready event
    emitter.on(events::BotReadyEvent, |_, ctx| async move {
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
