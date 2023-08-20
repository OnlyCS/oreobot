use crate::prelude::*;

pub fn button(disabled: bool) -> serenity::CreateButton {
    let mut button = serenity::CreateButton::default();

    button.custom_id("oreo_share");
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

    emitter.on_async_filter(
        events::ComponentInteractionEvent,
        |press, ctx| async move {
            let message = &press.message;

            press
                .create_interaction_response(&ctx, |resp| {
                    resp.interaction_response_data(|data| {
                        for embed in message.clone().embeds {
                            data.add_embed(embed.into());
                        }

                        data
                    })
                })
                .await?;

            message
                .clone()
                .edit(&ctx, |ed| {
                    let mut components = serenity::CreateComponents::default();

                    components.add_action_row(row(true));

                    ed.set_components(components)
                })
                .await?;

            Ok(())
        },
        |interaction| interaction.data.custom_id == "oreo_share",
    );

    Ok(())
}
