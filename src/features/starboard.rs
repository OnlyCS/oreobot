use crate::prelude::*;

pub async fn star(
    ctx: &serenity::Context,
    message: &serenity::Message,
) -> Result<serenity::Message> {
    let mut row = serenity::CreateActionRow::default();
    let mut delete_button = serenity::CreateButton::default();

    delete_button.style(serenity::ButtonStyle::Danger);
    delete_button.label("Admin: Remove Pin");
    delete_button.custom_id("oreo_starboard_delete");

    row.add_button(delete_button);

    let cloned = clone::clone(
        ctx,
        message,
        true,
        true,
        nci::channels::STARRED,
        vec![row],
        false,
    )
    .await?;

    Ok(cloned)
}

pub async fn register(ctx: &serenity::Context) -> Result<()> {
    let data = ctx.data.read().await;
    let emitter_mutex = Arc::clone(
        data.get::<EventEmitterTypeKey>()
            .context("Could not find event emitter")?,
    );

    let mut emitter = emitter_mutex.lock().await;

    emitter.on_async_filter(
        events::MessageReactionAdd,
        |payload, ctx| async move {
            let message = payload.message;
            let starred = star(&ctx, &message).await?;

            message
                .channel_id
                .send_message(&ctx, |msg| {
                    msg.reference_message(&message);

                    let mut embed = embed::serenity_default(&ctx, EmbedStatus::Sucess);
                    embed.title("Starboard");
                    embed.description("Message starred sucessfully");

                    let mut components = serenity::CreateComponents::default();
                    let mut row = serenity::CreateActionRow::default();
                    let mut btn = serenity::CreateButton::default();

                    btn.style(serenity::ButtonStyle::Link);
                    btn.label("Jump to starboard");
                    btn.url(starred.link());

                    row.add_button(btn);
                    components.add_action_row(row);
                    msg.set_components(components);
                    msg.set_embed(embed);

                    msg
                })
                .await?;

            Ok(())
        },
        |payload| {
            if let serenity::ReactionType::Unicode(ucode) = payload.reaction.emoji {
                ucode == emoji::PIN_EMOJI
            } else {
                false
            }
        },
    );

    emitter.on_async_filter(
        events::ComponentInteractionEvent,
        |interaction, ctx| async move {
            let author = &interaction.user;

            if !is_admin::user(&prisma::create().await?, author).await? {
                interaction
                    .create_interaction_response(&ctx, |resp| {
                        resp.interaction_response_data(|data| {
                            let mut embed = embed::serenity_default(&ctx, EmbedStatus::Warning);

                            embed.title("Starboard: Delete message");
                            embed.description("You do not have permission to delete this message.");
                            data.add_embed(embed).ephemeral(true)
                        })
                    })
                    .await?;
            }

            interaction
                .create_interaction_response(&ctx, |resp| {
                    resp.kind(serenity::InteractionResponseType::Modal)
                        .interaction_response_data(|resp_data| {
                            resp_data
                                .custom_id("oreo_starboard_delete_confirm")
                                .title("Starboard: Delete message")
                                .components(|create| {
                                    create.create_action_row(|action_row| {
                                        action_row.create_input_text(|input| {
                                            input
                                                .label("Reason for deletion (optional)")
                                                .required(false)
                                                .custom_id("oreo_starboard_delete_reason")
                                                .style(serenity::InputTextStyle::Short)
                                        })
                                    })
                                })
                        })
                })
                .await?;

            Ok(())
        },
        |interaction| interaction.data.custom_id == "oreo_starboard_delete",
    );

    emitter.on_async_filter(
        events::ModalInteractionEvent,
        |interaction, ctx| async move {
            let channel = interaction.channel_id;
            let webhooks = channel.webhooks(&ctx).await?;
            let wh = webhooks
                .into_iter()
                .find(|wh| wh.name == Some("Oreo's Internals".to_string()))
                .context("Could not find webhook")?;

            let message = interaction.message.as_ref().unwrap();

            wh.delete_message(&ctx, message.id).await?;

            interaction
                .create_interaction_response(&ctx, |resp| {
                    resp.interaction_response_data(|data| {
                        let mut embed = embed::serenity_default(&ctx, EmbedStatus::Sucess);

                        embed.title("Starboard: Delete message");
                        embed.description("Message deleted sucessfully.");

                        data.add_embed(embed).ephemeral(true)
                    })
                })
                .await?;

            Ok(())
        },
        |interaction| {
            interaction
                .data
                .custom_id
                .starts_with("oreo_starboard_delete_confirm")
        },
    );

    Ok(())
}
