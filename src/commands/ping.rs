use crate::prelude::*;

#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<()> {
    let created_timestamp = ctx.created_at().timestamp_millis();
    let now = serenity::Timestamp::now().timestamp_millis();
    let latency = now - created_timestamp;

    // bro where tf is client.ping();
    let api_latency = latency::api_ping(&ctx).await;

    ctx.send(|reply| {
        let mut embed = embed::default(&ctx, embed::EmbedStatus::Sucess);

        embed.title("The bot is up and running!");

        embed.field("Latency", format!("{}ms", latency), true);
        embed.field(
            "API Ping",
            if let Some(latency) = api_latency {
                format!("{}ms", latency.as_millis())
            } else {
                "Could not get API latency".to_string()
            },
            true,
        );

        reply.embed(|f| {
            f.clone_from(&embed);
            f
        });

        reply.components(|components| {
            components.add_action_row(share::row(false));

            components
        });

        reply.ephemeral(true);

        reply
    })
    .await?;

    Ok(())
}
