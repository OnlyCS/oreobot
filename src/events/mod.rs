pub mod emitter;
pub mod error;
pub mod event;
pub mod payload;

use crate::prelude::*;

pub async fn event_handler(ctx: serenity::Context, event: poise::Event<'_>) -> Result<()> {
    let emitter_mutex = Arc::clone(
        ctx.data
            .read()
            .await
            .get::<EventEmitterTypeKey>()
            .context("Could not find event emitter")?,
    );

    let mut event_emitter = emitter_mutex.lock().await;

    match event {
        /*** INTERACTION EVENTS ***/
        poise::Event::InteractionCreate { interaction } => match interaction {
            serenity::Interaction::ApplicationCommand(interaction) => {
                event_emitter
                    .emit(event::CommandInteractionEvent, interaction, &ctx)
                    .await?
            }
            serenity::Interaction::MessageComponent(interaction) => {
                event_emitter
                    .emit(event::ComponentInteractionEvent, interaction, &ctx)
                    .await?
            }
            serenity::Interaction::ModalSubmit(interaction) => {
                event_emitter
                    .emit(event::ModalInteractionEvent, interaction, &ctx)
                    .await?
            }
            _ => {}
        },

        /*** CATEGORY EVENTS ***/
        poise::Event::CategoryCreate { category } => {
            event_emitter
                .emit(event::CategoryCreateEvent, category.clone(), &ctx)
                .await?
        }
        poise::Event::CategoryDelete { category } => {
            event_emitter
                .emit(event::CategoryDeleteEvent, category.clone(), &ctx)
                .await?
        }

        /*** CHANNEL EVENTS ***/
        poise::Event::ChannelCreate { channel } => {
            event_emitter
                .emit(event::ChannelCreateEvent, channel.clone(), &ctx)
                .await?
        }
        poise::Event::ChannelUpdate { new: channel, .. } => match channel {
            serenity::Channel::Guild(channel) => {
                event_emitter
                    .emit(event::ChannelUpdateEvent, channel, &ctx)
                    .await?
            }
            serenity::Channel::Category(channel) => {
                event_emitter
                    .emit(event::CategoryUpdateEvent, channel, &ctx)
                    .await?
            }
            _ => {}
        },
        poise::Event::ChannelDelete { channel } => {
            event_emitter
                .emit(event::ChannelDeleteEvent, channel.clone(), &ctx)
                .await?
        }

        /*** MESSAGE EVENTS ***/
        poise::Event::Message {
            new_message: message,
        } => {
            event_emitter
                .emit(event::MessageCreateEvent, message, &ctx)
                .await?
        }
        poise::Event::MessageUpdate { event, .. } => {
            event_emitter
                .emit(event::MessageUpdateEvent, event, &ctx)
                .await?
        }
        poise::Event::MessageDelete {
            channel_id,
            deleted_message_id: message_id,
            guild_id,
        } => {
            let payload = payload::MessageDeletePayload {
                channel_id,
                message_id,
                guild_id,
            };

            event_emitter
                .emit(event::MessageDeleteEvent, payload, &ctx)
                .await?
        }
        poise::Event::MessageDeleteBulk {
            channel_id,
            multiple_deleted_messages_ids: message_ids,
            guild_id,
        } => {
            for message_id in message_ids {
                let payload = payload::MessageDeletePayload {
                    channel_id,
                    message_id,
                    guild_id,
                };

                event_emitter
                    .emit(event::MessageDeleteEvent, payload, &ctx)
                    .await?
            }
        }

        /*** ROLE EVENTS ***/
        poise::Event::GuildRoleCreate { new: role } => {
            event_emitter
                .emit(event::RoleCreateEvent, role, &ctx)
                .await?
        }
        poise::Event::GuildRoleUpdate { new: role, .. } => {
            event_emitter
                .emit(event::RoleUpdateEvent, role, &ctx)
                .await?
        }
        poise::Event::GuildRoleDelete {
            guild_id,
            removed_role_id: role_id,
            ..
        } => {
            let payload = payload::RoleDeletePayload { guild_id, role_id };

            event_emitter
                .emit(event::RoleDeleteEvent, payload, &ctx)
                .await?
        }

        /*** READY EVENTS ***/
        poise::Event::Ready {
            data_about_bot: ready,
        } => {
            event_emitter
                .emit(event::BotReadyEvent, ready, &ctx)
                .await?
        }

        _ => {}
    }

    Ok(())
}
