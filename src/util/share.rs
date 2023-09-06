use crate::prelude::*;

pub fn button(disabled: bool) -> serenity::CreateButton {
    let mut button = serenity::CreateButton::default();

    button.custom_id("oreo_share");
    button.label("Share");
    button.style(serenity::ButtonStyle::Secondary);
    button.disabled(disabled);

    button
}

fn unshare_button() -> serenity::CreateButton {
    let mut button = serenity::CreateButton::default();

    button.custom_id("oreo_unshare");
    button.label("Unshare");
    button.style(serenity::ButtonStyle::Danger);

    button
}

pub fn row(disabled: bool) -> serenity::CreateActionRow {
    let mut row = serenity::CreateActionRow::default();

    row.add_button(button(disabled));

    row
}

fn unshare_row() -> serenity::CreateActionRow {
    let mut row = serenity::CreateActionRow::default();

    row.add_button(unshare_button());

    row
}

pub async fn register(ctx: &serenity::Context) {
    let data_arc = data::get_serenity(ctx).await;
    let mut data = data_arc.lock().await;
    let emitter = &mut data.emitter;

    emitter.on_filter(
        events::ComponentInteractionEvent,
        |press, ctx| async move {
            let message = &press.message;

            press
                .create_interaction_response(&ctx, |resp| {
                    resp.interaction_response_data(|data| {
                        for embed in message.clone().embeds {
                            data.add_embed(embed.into());
                        }

                        data.set_components(
                            serenity::CreateComponents::default()
                                .set_action_row(unshare_row())
                                .clone(),
                        );

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

    emitter.on_filter(
        events::ComponentInteractionEvent,
        |press, ctx| async move {
            let message = &press.message;

            if !is_admin::user_id(&prisma::create().await?, &press.user.id).await? {
                press
                    .create_interaction_response(&ctx, |resp| {
                        resp.interaction_response_data(|data| {
                            data.ephemeral(true);

                            let mut embed = embed::serenity_default(&ctx, EmbedStatus::Warning);

                            embed.title("Not Admin");
                            embed.description("You must be an admin to unshare this message");

                            data.add_embed(embed);

                            data.set_components(
                                serenity::CreateComponents::default()
                                    .add_action_row(row(false))
                                    .clone(),
                            );

                            data
                        })
                    })
                    .await?;

                return Ok(());
            }

            message.delete(&ctx).await?;

            press
                .create_interaction_response(&ctx, |resp| {
                    resp.interaction_response_data(|data| {
                        data.ephemeral(true);

                        let mut embed = embed::serenity_default(&ctx, EmbedStatus::Sucess);

                        embed.title("Unshare");
                        embed.description("Message sucessfully unsahred");

                        data.set_components(
                            serenity::CreateComponents::default()
                                .add_action_row(row(false))
                                .clone(),
                        );

                        data.add_embed(embed);

                        data
                    })
                })
                .await?;

            Ok(())
        },
        |interaction| interaction.data.custom_id == "oreo_unshare",
    );
}
