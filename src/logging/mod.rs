mod category;
mod channel;
mod interaction;
mod member;
mod message;
mod role;

pub mod ready;

use crate::prelude::*;

pub async fn register(ctx: &serenity::Context) {
    let data_arc = data::get_serenity(ctx).await;
    let mut data = data_arc.lock().await;
    let emitter = &mut data.emitter;

    // interaction events
    emitter.on(
        events::CommandInteractionEvent,
        |interaction, _| async move { Ok(interaction::command(interaction).await?) },
    );

    emitter.on(
        events::ComponentInteractionEvent,
        |interaction, _| async move { Ok(interaction::message_component(interaction).await?) },
    );

    emitter.on(events::ModalInteractionEvent, |interaction, _| async move {
        Ok(interaction::modal_submit(interaction).await?)
    });

    // category events
    emitter.on(events::CategoryCreateEvent, |category, _| async move {
        Ok(category::create(category).await?)
    });

    emitter.on(events::CategoryUpdateEvent, |category, _| async move {
        Ok(category::update(category).await?)
    });

    emitter.on(events::CategoryDeleteEvent, |category, _| async move {
        Ok(category::delete(category.id).await?)
    });

    // channel events
    emitter.on(events::ChannelCreateEvent, |channel, _| async move {
        match channel::create(channel.clone()).await {
            Ok(_) => Ok(()),
            Err(LoggingError::ChannelIsThread(_)) => {
                warn!("Channel {} is a thread, ignoring", channel.id);
                Ok(())
            }
            Err(e) => Err(e)?,
        }
    });

    emitter.on(events::ChannelUpdateEvent, |channel, _| async move {
        match channel::update(channel.clone()).await {
            Ok(_) => Ok(()),
            Err(LoggingError::ChannelIsThread(_)) => {
                warn!("Channel {} is a thread, ignoring", channel.id);
                Ok(())
            }
            Err(e) => Err(e)?,
        }
    });

    emitter.on(events::ChannelDeleteEvent, |channel, _| async move {
        match channel::delete(channel.id).await {
            Ok(_) => Ok(()),
            Err(LoggingError::ChannelIsThread(_)) => {
                warn!("Channel {} is a thread, ignoring", channel.id);
                Ok(())
            }
            Err(e) => Err(e)?,
        }
    });

    // message events
    emitter.on(events::MessageCreateEvent, |message, ctx| async move {
        Ok(message::create(ctx, message).await?)
    });

    emitter.on(events::MessageUpdateEvent, |event, _| async move {
        Ok(message::update(event).await?)
    });

    emitter.on(events::MessageDeleteEvent, |payload, _| async move {
        Ok(message::delete(payload.message_id).await?)
    });

    // role events
    emitter.on(events::RoleCreateEvent, |role, _| async move {
        Ok(role::create(role).await?)
    });

    emitter.on(events::RoleUpdateEvent, |role, _| async move {
        Ok(role::update(role).await?)
    });

    emitter.on(events::RoleDeleteEvent, |payload, ctx| async move {
        Ok(role::delete(payload.role_id, ctx).await?)
    });

    // member events
    emitter.on(events::MemberJoinEvent, |member, ctx| async move {
        Ok(member::join(member, ctx).await?)
    });

    emitter.on(events::MemberUpdateEvent, |member, ctx| async move {
        Ok(member::update(member, ctx).await?)
    });

    emitter.on(events::MemberLeaveEvent, |payload, ctx| async move {
        Ok(member::leave(payload.user.id, ctx).await?)
    });

    // ready event
    emitter.on(events::BotReadyEvent, |_, ctx| async move {
        ctx.set_presence(
            Some(serenity::Activity::playing("with Oppenheimer")),
            serenity::OnlineStatus::Online,
        )
        .await;

        ready::on_ready(ctx.clone()).await?;

        Ok(())
    });
}
