use crate::prelude::*;

async fn _star(
    no_interaction: Option<&serenity::Context>,
    interaction: Option<&Context<'_>>,
    message: &serenity::Message,
) -> Result<()> {
    let prisma = prisma::create().await?;
    let message_data = prisma
        .message()
        .find_unique(message::id::equals(message.id.to_string()))
        .exec()
        .await?
        .context("Could not find message in database")?;

    let is_pinned = {
        if message_data.pinned {
            !prisma
                .message_pin()
                .find_unique(message_pin::original_id::equals(message_data.clone().id))
                .exec()
                .await?
                .map(|n| n.removed)
                .unwrap_or(false)
        } else {
            message_data.pinned
        }
    };

    if is_pinned {
        let mut embed = if let Some(ctx) = no_interaction {
            embed::serenity_default(&ctx, EmbedStatus::Warning)
        } else if let Some(ctx) = interaction {
            embed::default(ctx, EmbedStatus::Warning)
        } else {
            unreachable!("No context provided")
        };

        embed.title("Starboard");
        embed.description("This message is already pinned.");

        if let Some(ctx) = no_interaction {
            message
                .channel_id
                .send_message(&ctx, |reply| {
                    reply
                        .reference_message(message)
                        .allowed_mentions(|mention| mention.replied_user(false));

                    reply.set_embed(embed);

                    reply
                })
                .await?;
        } else if let Some(ctx) = interaction {
            ctx.send(|reply| {
                reply.ephemeral(true);
                reply.embeds.push(embed);

                reply
            })
            .await?;
        }

        return Ok(());
    } else {
        prisma
            .message()
            .update(
                message::id::equals(message.id.to_string()),
                vec![message::pinned::set(true)],
            )
            .exec()
            .await?;

        if message_data.pinned {
            prisma
                .message_pin()
                .delete(message_pin::original_id::equals(message_data.id))
                .exec()
                .await?;
        }
    }

    let mut row = serenity::CreateActionRow::default();
    let mut delete_button = serenity::CreateButton::default();

    delete_button.style(serenity::ButtonStyle::Danger);
    delete_button.label("Admin: Remove Pin");
    delete_button.custom_id("oreo_starboard_delete");

    row.add_button(delete_button);

    let cloned = clone::clone(
        if let Some(ctx) = no_interaction {
            ctx
        } else if let Some(ctx) = interaction {
            ctx.serenity_context()
        } else {
            unreachable!("no context")
        },
        message,
        true,
        true,
        nci::channels::STARRED,
        vec![row],
        false,
    )
    .await?;

    prisma
        .message_pin()
        .create(
            cloned.id.to_string(),
            message::id::equals(message.id.to_string()),
            vec![],
        )
        .exec()
        .await?;

    let mut embed = if let Some(ctx) = no_interaction {
        embed::serenity_default(&ctx, EmbedStatus::Sucess)
    } else if let Some(ctx) = interaction {
        embed::default(ctx, EmbedStatus::Sucess)
    } else {
        unreachable!("No context provided")
    };

    embed.title("Starboard");
    embed.description("Message starred sucessfully");

    let mut components = serenity::CreateComponents::default();
    let mut row = serenity::CreateActionRow::default();
    let mut btn = serenity::CreateButton::default();

    btn.style(serenity::ButtonStyle::Link);
    btn.label("Jump to starboard");
    btn.url(cloned.link());

    row.add_button(btn);
    components.add_action_row(row);

    if let Some(ctx) = no_interaction {
        message
            .channel_id
            .send_message(&ctx, |reply| {
                reply
                    .reference_message(message)
                    .allowed_mentions(|mention| mention.replied_user(false));

                reply.set_components(components);
                reply.set_embed(embed);

                reply
            })
            .await?;
    } else if let Some(ctx) = interaction {
        ctx.send(|reply| {
            reply.ephemeral(true);

            reply.components = Some(components);
            reply.embeds.push(embed);

            reply
        })
        .await?;
    }

    Ok(())
}

pub async fn star_no_interaction(
    ctx: &serenity::Context,
    message: &serenity::Message,
) -> Result<()> {
    _star(Some(ctx), None, message).await
}

pub async fn star_interaction(ctx: &Context<'_>, message: &serenity::Message) -> Result<()> {
    _star(None, Some(ctx), message).await
}

pub async fn register(ctx: &serenity::Context) -> Result<()> {
    let data = ctx.data.read().await;
    let emitter_mutex = Arc::clone(
        data.get::<EventEmitterTypeKey>()
            .context("Could not find event emitter")?,
    );

    let mut emitter = emitter_mutex.lock().await;

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
                .context("Could not find webhook")?;

            let reason = interaction
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
                .context("Could not find reason component")?
                .clone()
                .value;

            prisma
                .message_pin()
                .update(
                    message_pin::pinned_message_id::equals(message.id.to_string()),
                    vec![
                        message_pin::removed::set(true),
                        message_pin::removed_reason::set(Some(reason)),
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
