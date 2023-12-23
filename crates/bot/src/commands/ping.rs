use crate::prelude::*;

/// Ping the bot to see if it's alive.
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), CommandError> {
    let sent = ctx.created_at().timestamp_millis();
    let now = serenity::Timestamp::now().timestamp_millis();

    let latency = now - sent;
    let api_latency = ctx.ping().await.as_millis();

    let embed = embed::default(EmbedStatus::Success)
        .title("The bot is up and running!")
        .field("Latency", format!("{latency}ms"), true)
        .field("API Ping", format!("{api_latency}ms"), true);

    let reply = poise::CreateReply::default()
        .embed(embed)
        .components(vec![share::row()])
        .ephemeral(true);

    ctx.send(reply).await?;

    Ok(())
}
