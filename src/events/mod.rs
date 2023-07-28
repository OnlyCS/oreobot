use crate::prelude::*;

mod category;
mod channel;
mod interaction;
mod message;

pub async fn event_handler(ctx: serenity::Context, event: poise::Event<'_>) -> Result<()> {
    match event {
        /*** MESSAGE EVENTS ***/
        poise::Event::Message { new_message } => {
            let prisma_client = prisma::create().await?;

            if new_message
                .channel(ctx)
                .await?
                .guild()
                .map(|n| n.is_thread())
                .unwrap_or(false)
            {
                return Ok(()); // to hell with all threads
            }

            message::create(new_message, &prisma_client).await?;
        }
        poise::Event::MessageUpdate {
            event,
            old_if_available: _,
            new: _,
        } => {
            let prisma_client = prisma::create().await?;

            message::update(event, &prisma_client).await?;
        }
        poise::Event::MessageDelete {
            channel_id: _,
            deleted_message_id,
            guild_id: _,
        } => {
            let prisma_client = prisma::create().await?;

            message::delete(deleted_message_id, &prisma_client).await?;
        }
        poise::Event::MessageDeleteBulk {
            channel_id: _,
            multiple_deleted_messages_ids,
            guild_id: _,
        } => {
            let prisma_client = prisma::create().await?;

            for id in multiple_deleted_messages_ids {
                message::delete(id, &prisma_client).await?;
            }
        }

        /*** CHANNEL EVENTS ***/
        poise::Event::ChannelCreate {
            channel: new_channel,
        } => {
            let prisma_client = prisma::create().await?;

            channel::create(new_channel, &prisma_client).await?
        }
        poise::Event::ChannelUpdate { old: _, new } => {
            let prisma_client = prisma::create().await?;

            if let Some(channel) = new.clone().guild() {
                if channel.is_thread() {
                    return Ok(());
                }

                channel::update(channel, &prisma_client).await?;
            } else if let Some(category) = new.category() {
                category::update(category, &prisma_client).await?;
            }
        }
        poise::Event::ChannelDelete { channel: deleted } => {
            let prisma_client = prisma::create().await?;

            let id = deleted.id;
            channel::delete(id, &prisma_client).await.unwrap();
        }

        /*** CATEGORY EVENTS ***/
        poise::Event::CategoryCreate { category } => {
            let prisma_client = prisma::create().await?;

            category::create(category, &prisma_client).await?;
        }
        poise::Event::CategoryDelete { category } => {
            let prisma_client = prisma::create().await?;

            category::delete(category.id, &prisma_client).await?;
        }

        /*** INTERACTION EVENTS ***/
        poise::Event::InteractionCreate { interaction } => {
            let prisma_client = prisma::create().await?;

            interaction::create(interaction, &prisma_client).await?;
        }
        _ => {}
    }

    Ok(())
}
