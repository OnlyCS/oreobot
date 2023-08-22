use crate::prelude::*;

pub async fn star(
    ctx: &serenity::Context,
    message: &serenity::Message,
) -> Result<serenity::Message> {
    let mut delete_button = serenity::CreateButton::default();
    let mut update_button = serenity::CreateButton::default();
    let message_id = message.id.to_string();

    delete_button.style(serenity::ButtonStyle::Danger);
    delete_button.label("Admin: Remove Pin");
    delete_button.custom_id(format!("oreo_starboard_delete_{}", message_id));

    update_button.style(serenity::ButtonStyle::Primary);
    update_button.label("Admin: Update Pin");
    update_button.custom_id(format!("oreo_starboard_update_{}", message_id));

    let mut row = serenity::CreateActionRow::default();

    row.add_button(delete_button);
    row.add_button(update_button);

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

    emitter.on_async(events::MessageReactionAdd, |reaction, ctx| async move {
        let message = reaction.message(&ctx).await?;
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
    });

    Ok(())
}
