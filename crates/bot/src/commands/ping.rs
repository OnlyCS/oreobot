use crate::prelude::*;

#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), CommandError> {
    ctx.reply("Pong!").await?;

    Ok(())
}
