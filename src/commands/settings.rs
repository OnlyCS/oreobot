use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, SelectMenuOptions, Hash)]
pub enum UserSetting {
    #[label = "Pin Confirmation"]
    #[ty = "bool"]
    PinConfirm,
}

#[poise::command(slash_command)]
pub async fn settings(ctx: Context<'_>) -> Result<(), CommandError> {
    let mut components = serenity::CreateComponents::default();
    let mut row = serenity::CreateActionRow::default();
    let mut string_select = serenity::CreateSelectMenu::default();

    string_select.custom_id("oreo_setting_name");
    string_select.placeholder("Select a setting to change");
    string_select.options(|o| o.set_options(UserSetting::options()));
    string_select.min_values(1);
    string_select.max_values(1);

    row.add_select_menu(string_select);
    components.add_action_row(row);
    components.add_action_row(share::row(false));

    let mut embed = embed::default(&ctx, EmbedStatus::Sucess);
    embed.title("Settings");
    embed.description("Select a setting to change");

    ctx.send(|reply| {
        reply.embeds.push(embed);
        reply.components = Some(components);
        reply.ephemeral(true);
        reply
    })
    .await?;

    Ok(())
}

pub async fn register(ctx: &serenity::Context) {
    let data_arc = data::get_serenity(&ctx).await;
    let mut data = data_arc.lock().await;
    let emitter = &mut data.emitter;

    emitter.on_filter(
        events::ComponentInteractionEvent,
        |interaction, ctx| async move {
            let value = interaction.data.values.first().unwrap();
            let setting = UserSetting::from_str(value)?;
            let row = setting.type_row(format!("oreo_setting_value_{}", value));

            let mut components = serenity::CreateComponents::default();
            components.add_action_row(row);
            components.add_action_row(share::row(false));

            let mut embed = embed::default(&ctx, EmbedStatus::Sucess);
            embed.title("Settings");
            embed.description("Change value of setting");

            interaction
                .create_interaction_response(&ctx, |response| {
                    response.interaction_response_data(|data| {
                        data.add_embed(embed).set_components(components)
                    })
                })
                .await?;

            Ok(())
        },
        |interaction| interaction.data.custom_id == "oreo_setting_name",
    );

    emitter.on_filter(
        events::ComponentInteractionEvent,
        |interaction, ctx| async move {
            let value = interaction
                .data
                .custom_id
                .trim_start_matches("oreo_setting_value_");

            let setting = UserSetting::from_str(value)?;

            let data_arc = data::get_serenity(&ctx).await;
            let mut data = data_arc.lock().await;
            let cache = &mut data.cache;

            match setting {
                UserSetting::PinConfirm => {
                    let boolean_option = interaction.data.values.first().unwrap();
                    let boolean_value = setting.parse_as_bool(boolean_option)?;

                    cache
                        .update::<cache_items::UserSettings>(
                            ctx,
                            interaction.user.id,
                            SettingsDataUpdate::PinConfirm(boolean_value),
                        )
                        .await?;
                }
            }

            Ok(())
        },
        |interaction| {
            interaction
                .data
                .custom_id
                .starts_with("oreo_setting_value_")
        },
    )
}
