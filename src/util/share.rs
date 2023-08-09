use crate::prelude::*;

pub fn button(disabled: bool) -> serenity::CreateButton {
    let mut button = serenity::CreateButton::default();

    button.custom_id("share");
    button.label("Share");
    button.style(serenity::ButtonStyle::Secondary);
    button.disabled(disabled);

    button
}

pub fn row(disabled: bool) -> serenity::CreateActionRow {
    let mut row = serenity::CreateActionRow::default();

    row.add_button(button(disabled));

    row
}

pub async fn register(ctx: &serenity::Context) -> Result<()> {
    let data = ctx.data.read().await;
    let emitter_mutex = Arc::clone(
        data.get::<EventEmitterTypeKey>()
            .context("Could not find event emitter")?,
    );

    let mut emitter = emitter_mutex.lock().await;

    emitter.on_async(
        events::CommandInteractionEvent,
        |interaction, ctx| async move {
            info!("Got command interaction");

            let message = interaction.get_interaction_response(&ctx).await?;

            let press = message
                .await_component_interaction(ctx.clone())
                .filter(|component| component.data.custom_id == "share".to_string())
                .await
                .context("Could not get interaction")?;

            press
                .create_interaction_response(&ctx, |resp| {
                    resp.interaction_response_data(|data| {
                        for embed in message.embeds {
                            data.add_embed(embed.into());
                        }

                        data
                    })
                })
                .await?;

            interaction
                .edit_original_interaction_response(&ctx, |resp| {
                    resp.components(|comp| comp.set_action_row(row(true)))
                })
                .await?;

            Ok(())
        },
    );

    Ok(())
}
