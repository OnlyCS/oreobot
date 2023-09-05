pub mod emitter;
pub mod error;
pub mod events;
pub mod payloads;

use crate::prelude::*;

pub async fn event_handler(ctx: serenity::Context, event: poise::Event<'_>) -> Result<()> {
    let data_arc = data::get_serenity(&ctx).await?;
    let mut data = data_arc.lock().await;
    let event_emitter = &mut data.emitter;

    match event {
        /*** INTERACTION EVENTS ***/
        poise::Event::InteractionCreate { interaction } => match interaction {
            serenity::Interaction::ApplicationCommand(interaction) => {
                event_emitter
                    .emit(events::CommandInteractionEvent, interaction, &ctx)
                    .await?
            }
            serenity::Interaction::MessageComponent(interaction) => {
                event_emitter
                    .emit(events::ComponentInteractionEvent, interaction, &ctx)
                    .await?
            }
            serenity::Interaction::ModalSubmit(interaction) => {
                event_emitter
                    .emit(events::ModalInteractionEvent, interaction, &ctx)
                    .await?
            }
            _ => {}
        },

        /*** CATEGORY EVENTS ***/
        poise::Event::CategoryCreate { category } => {
            event_emitter
                .emit(events::CategoryCreateEvent, category.clone(), &ctx)
                .await?
        }
        poise::Event::CategoryDelete { category } => {
            event_emitter
                .emit(events::CategoryDeleteEvent, category.clone(), &ctx)
                .await?
        }

        /*** CHANNEL EVENTS ***/
        poise::Event::ChannelCreate { channel } => {
            event_emitter
                .emit(events::ChannelCreateEvent, channel.clone(), &ctx)
                .await?
        }
        poise::Event::ChannelUpdate { new: channel, .. } => match channel {
            serenity::Channel::Guild(channel) => {
                event_emitter
                    .emit(events::ChannelUpdateEvent, channel, &ctx)
                    .await?
            }
            serenity::Channel::Category(channel) => {
                event_emitter
                    .emit(events::CategoryUpdateEvent, channel, &ctx)
                    .await?
            }
            _ => {}
        },
        poise::Event::ChannelDelete { channel } => {
            event_emitter
                .emit(events::ChannelDeleteEvent, channel.clone(), &ctx)
                .await?
        }

        /*** MESSAGE EVENTS ***/
        poise::Event::Message {
            new_message: message,
        } => {
            event_emitter
                .emit(events::MessageCreateEvent, message, &ctx)
                .await?
        }
        poise::Event::MessageUpdate { event, .. } => {
            event_emitter
                .emit(events::MessageUpdateEvent, event, &ctx)
                .await?
        }
        poise::Event::MessageDelete {
            channel_id,
            deleted_message_id: message_id,
            guild_id,
        } => {
            let payload = payloads::MessageDeletePayload {
                channel_id,
                message_id,
                guild_id,
            };

            event_emitter
                .emit(events::MessageDeleteEvent, payload, &ctx)
                .await?
        }
        poise::Event::MessageDeleteBulk {
            channel_id,
            multiple_deleted_messages_ids: message_ids,
            guild_id,
        } => {
            for message_id in message_ids {
                let payload = payloads::MessageDeletePayload {
                    channel_id,
                    message_id,
                    guild_id,
                };

                event_emitter
                    .emit(events::MessageDeleteEvent, payload, &ctx)
                    .await?
            }
        }
        poise::Event::ReactionAdd { add_reaction } => {
            let message = add_reaction.message(&ctx).await.unwrap();

            event_emitter
                .emit(
                    events::MessageReactionAdd,
                    payloads::MessageReactionAddPayload {
                        reaction: add_reaction,
                        message,
                    },
                    &ctx,
                )
                .await?
        }

        /*** ROLE EVENTS ***/
        poise::Event::GuildRoleCreate { new: role } => {
            event_emitter
                .emit(events::RoleCreateEvent, role, &ctx)
                .await?
        }
        poise::Event::GuildRoleUpdate { new: role, .. } => {
            event_emitter
                .emit(events::RoleUpdateEvent, role, &ctx)
                .await?
        }
        poise::Event::GuildRoleDelete {
            guild_id,
            removed_role_id: role_id,
            ..
        } => {
            let payload = payloads::RoleDeletePayload { guild_id, role_id };

            event_emitter
                .emit(events::RoleDeleteEvent, payload, &ctx)
                .await?
        }

        /*** MEMBER EVENTS ***/
        poise::Event::GuildMemberAddition { new_member: member } => {
            event_emitter
                .emit(events::MemberJoinEvent, member, &ctx)
                .await?
        }
        poise::Event::GuildMemberUpdate { new, .. } => {
            event_emitter
                .emit(events::MemberUpdateEvent, new, &ctx)
                .await?
        }
        poise::Event::GuildMemberRemoval { guild_id, user, .. } => {
            let payload = payloads::MemberLeavePayload { guild_id, user };

            event_emitter
                .emit(events::MemberLeaveEvent, payload, &ctx)
                .await?
        }

        /*** READY EVENTS ***/
        poise::Event::Ready {
            data_about_bot: ready,
        } => {
            event_emitter
                .emit(events::BotReadyEvent, ready, &ctx)
                .await?
        }

        _ => {}
    }

    Ok(())
}
