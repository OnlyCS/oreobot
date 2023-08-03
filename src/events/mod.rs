pub mod emitter;
pub mod error;
pub mod payloads;

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
        /*** MESSAGE EVENTS ***/
        poise::Event::Message { new_message } => {
            event_emitter
                .emit(
                    &EmitterEvent::MessageCreate,
                    MessageCreatePayload::from(new_message),
                    &ctx,
                )
                .await?;
        }
        poise::Event::MessageUpdate {
            event,
            old_if_available: _,
            new: _,
        } => {
            let payload = MessageUpdatePayload::from(event.clone());

            event_emitter
                .emit(
                    &EmitterEvent::MessageUpdate { id: event.id },
                    payload.clone(),
                    &ctx,
                )
                .await?;

            event_emitter
                .emit(&EmitterEvent::AnyMessageUpdate, payload.clone(), &ctx)
                .await?;
        }
        poise::Event::MessageDelete {
            channel_id,
            deleted_message_id: message_id,
            guild_id,
        } => {
            let payload = MessageDeletePayload::from((guild_id, channel_id, message_id));

            event_emitter
                .emit(
                    &EmitterEvent::MessageDelete { id: message_id },
                    payload.clone(),
                    &ctx,
                )
                .await?;

            event_emitter
                .emit(&EmitterEvent::AnyMessageDelete, payload.clone(), &ctx)
                .await?;
        }
        poise::Event::MessageDeleteBulk {
            channel_id,
            multiple_deleted_messages_ids: message_ids,
            guild_id,
        } => {
            for id in message_ids {
                let payload = MessageDeletePayload::from((guild_id, channel_id, id));

                event_emitter
                    .emit(&EmitterEvent::MessageDelete { id }, payload.clone(), &ctx)
                    .await?;

                event_emitter
                    .emit(&EmitterEvent::AnyMessageDelete, payload.clone(), &ctx)
                    .await?;
            }
        }

        /*** CHANNEL EVENTS ***/
        poise::Event::ChannelCreate { channel } => {
            event_emitter
                .emit(
                    &EmitterEvent::ChannelCreate,
                    ChannelCreatePayload::from(channel.clone()),
                    &ctx,
                )
                .await?;
        }
        poise::Event::ChannelUpdate { old: _, new } => {
            if let Some(channel) = new.clone().guild() {
                event_emitter
                    .emit(
                        &EmitterEvent::ChannelUpdate { id: channel.id },
                        ChannelUpdatePayload::from(channel.clone()),
                        &ctx,
                    )
                    .await?;

                event_emitter
                    .emit(
                        &EmitterEvent::AnyChannelUpdate,
                        ChannelUpdatePayload::from(channel.clone()),
                        &ctx,
                    )
                    .await?;
            } else if let Some(category) = new.category() {
                event_emitter
                    .emit(
                        &EmitterEvent::CategoryUpdate { id: category.id },
                        CategoryUpdatePayload::from(category.clone()),
                        &ctx,
                    )
                    .await?;

                event_emitter
                    .emit(
                        &EmitterEvent::AnyCategoryUpdate,
                        CategoryUpdatePayload::from(category.clone()),
                        &ctx,
                    )
                    .await?;
            }
        }
        poise::Event::ChannelDelete { channel } => {
            let payload = ChannelDeletePayload::from(channel.clone());

            event_emitter
                .emit(
                    &EmitterEvent::ChannelDelete { id: channel.id },
                    payload.clone(),
                    &ctx,
                )
                .await?;

            event_emitter
                .emit(&EmitterEvent::AnyChannelDelete, payload, &ctx)
                .await?;
        }

        /*** CATEGORY EVENTS ***/
        poise::Event::CategoryCreate { category } => {
            event_emitter
                .emit(
                    &EmitterEvent::CategoryCreate,
                    CategoryCreatePayload::from(category.clone()),
                    &ctx,
                )
                .await?;
        }
        poise::Event::CategoryDelete { category } => {
            let payload = CategoryDeletePayload::from(category.clone());

            event_emitter
                .emit(
                    &EmitterEvent::CategoryDelete { id: category.id },
                    payload.clone(),
                    &ctx,
                )
                .await?;

            event_emitter
                .emit(&EmitterEvent::AnyCategoryDelete, payload, &ctx)
                .await?;
        }

        /*** INTERACTION EVENTS ***/
        poise::Event::InteractionCreate { interaction } => {
            if let Some(interaction) = interaction.clone().message_component() {
                let payload = ComponentInteractionPayload::from(interaction.clone());

                event_emitter
                    .emit(
                        &EmitterEvent::ComponentInteraction {
                            custom_id: interaction.data.custom_id.clone(),
                        },
                        payload.clone(),
                        &ctx,
                    )
                    .await?;

                event_emitter
                    .emit(&EmitterEvent::AnyComponentInteraction, payload, &ctx)
                    .await?;
            }

            let payload = AnyInteractionPayload::from(interaction.clone());

            event_emitter
                .emit(&EmitterEvent::AnyInteraction, payload, &ctx)
                .await?;
        }

        /**** READY ****/
        poise::Event::Ready {
            data_about_bot: ready,
        } => {
            event_emitter
                .emit(&EmitterEvent::Ready, ReadyEventPayload::from(ready), &ctx)
                .await?;
        }
        _ => {}
    }

    Ok(())
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum EmitterEvent {
    ComponentInteraction { custom_id: String },
    AnyComponentInteraction,
    AnyInteraction,

    CategoryCreate,
    CategoryUpdate { id: serenity::ChannelId },
    AnyCategoryUpdate,
    CategoryDelete { id: serenity::ChannelId },
    AnyCategoryDelete,

    ChannelCreate,
    ChannelUpdate { id: serenity::ChannelId },
    AnyChannelUpdate,
    ChannelDelete { id: serenity::ChannelId },
    AnyChannelDelete,

    MessageCreate,
    MessageUpdate { id: serenity::MessageId },
    AnyMessageUpdate,
    MessageDelete { id: serenity::MessageId },
    AnyMessageDelete,

    Ready,
}
