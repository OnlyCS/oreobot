use crate::prelude::*;

#[poise::command(prefix_command, slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<()> {
    ctx.send(|r| {
        r.embed(|e| {
            e.make_default(ctx.data().clone())
                .data("User pinged me!", "Pong!")
        })
    })
    .await?;

    Ok(())
}
