use crate::prelude::*;

macro_rules! star {
    ($ctx:expr, $message:expr, $loading:expr, $seren:expr) => {{
        let prisma = prisma::create().await?;
        let ctx = $ctx;
        let loading = $loading;
        let message = $message;

        let message_data = prisma
            .message()
            .find_unique(message::id::equals(message.id.to_string()))
            .with(message::pin::fetch())
            .exec()
            .await?
            .make_error(StarboardError::MessageNotInDatabase(message.id))?;

        let existing_message_pin = message_data.pin()?;
        let is_pinned = existing_message_pin.map(|n| !n.removed).unwrap_or(false);

        if is_pinned {
            let mut embed = embed::default(&ctx, EmbedStatus::Warning);

            embed.title("Starboard");
            embed.description("This message is already pinned.");

            loading.last(&ctx, embed).await?;

            return Ok(());
        }

        let mut row = serenity::CreateActionRow::default();
        let mut delete_button = serenity::CreateButton::default();

        delete_button.style(serenity::ButtonStyle::Danger);
        delete_button.label("Admin: Remove Pin");
        delete_button.custom_id("oreo_starboard_delete");

        row.add_button(delete_button);

        let cloned = clone::clone(
            $seren,
            message,
            true,
            true,
            nci::channels::STARRED,
            vec![row],
            false,
            None,
        )
        .await?;

        if let Some(pin) = existing_message_pin {
            prisma
                .message_pin()
                .update(
                    message_pin::pinned_message_id::equals(pin.pinned_message_id.clone()),
                    vec![
                        message_pin::removed::set(false),
                        message_pin::removed_reason::set(None),
                        message_pin::original::connect(message::id::equals(message.id.to_string())),
                    ],
                )
                .exec()
                .await?;
        } else {
            prisma
                .message_pin()
                .create(
                    cloned.id.to_string(),
                    message::id::equals(message.id.to_string()),
                    vec![],
                )
                .exec()
                .await?;
        }

        let mut clone_finish = embed::default(&ctx, EmbedStatus::Sucess);

        clone_finish.title("Starboard");
        clone_finish.description("Message pinned sucessfully.");

        loading.last(&ctx, clone_finish).await?;

        Ok(())
    }};
}

pub async fn star_no_interaction(
    ctx: &serenity::Context,
    message: &serenity::Message,
) -> Result<(), StarboardError> {
    let loading =
        Loading::<LoadingWithoutInteraction>::new(ctx, message.channel_id, "Starring Message...")
            .await?;

    star!(ctx, message, loading, ctx)
}

pub async fn star_interaction(
    ctx: &Context<'_>,
    message: &serenity::Message,
) -> Result<(), StarboardError> {
    let loading = Loading::<LoadingWithInteraction>::new(ctx, "Starring Message...").await?;

    star!(ctx, message, loading, ctx.serenity_context())
}

pub async fn register(ctx: &serenity::Context) -> Result<(), StarboardError> {
    let data_arc = data::get_serenity(ctx).await;
    let mut data = data_arc.lock().await;
    let emitter = &mut data.emitter;

    emitter.on_filter(
        events::MessageReactionAdd,
        |payload, ctx| async move {
            star_no_interaction(&ctx, &payload.message).await?;
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

    emitter.on_filter(
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

    emitter.on_filter(
        events::ModalInteractionEvent,
        |interaction, ctx| async move {
            let prisma = prisma::create().await?;
            let channel = interaction.channel_id;
            let webhooks = channel.webhooks(&ctx).await?;
            let message = interaction.message.as_ref().unwrap();

            let wh = webhooks
                .into_iter()
                .find(|wh| wh.name == Some("Oreo's Internals".to_string()))
                .make_error(anyhow!("Could not find webhook"))?;

            let reason = &interaction
                .data
                .components
                .iter()
                .map(|row| {
                    row.components
                        .iter()
                        .filter_map(|component| match component {
                            serenity::ActionRowComponent::InputText(component) => Some(component),
                            _ => None,
                        })
                })
                .flatten()
                .next()
                .unwrap()
                .value;

            prisma
                .message_pin()
                .update(
                    message_pin::pinned_message_id::equals(message.id.to_string()),
                    vec![
                        message_pin::removed::set(true),
                        message_pin::removed_reason::set(Some(reason.clone())),
                    ],
                )
                .exec()
                .await?;

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
