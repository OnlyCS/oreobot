use crate::prelude::*;
mod ping;

pub async fn run() -> Result<()> {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping::ping()],
            ..Default::default()
        })
        .token(dotenv!("DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::all())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    framework.run().await?;
    Ok(())
}
