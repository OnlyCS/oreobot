use crate::prelude::*;

#[poise::command(context_menu_command = "Star Message")]
pub async fn star(ctx: Context<'_>, message: serenity::Message) -> Result<(), CommandError> {
    if message.channel_id == nci::channels::STARRED {
        Err(CommandError::RuntimeError {
            title: "Cannot star",
            description: "Cannot star a message that is in the starred channel",
        })?;
    }

    starboard::star_interaction(&ctx, &message).await?;

    let mut confirmation = embed::default(&ctx, EmbedStatus::Success);
    confirmation.title("Starboard > Star");
    confirmation.description("Sucessfully starred message");

    ctx.send(|m| {
        m.embeds.push(confirmation);
        m.ephemeral(true)
    })
    .await?;

    Ok(())
}
