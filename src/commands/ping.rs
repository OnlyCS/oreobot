use crate::prelude::*;

#[poise::command(prefix_command, slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<()> {
    ctx.say("Pong!").await?;
    Ok(())
}
