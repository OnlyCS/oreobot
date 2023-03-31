#![allow(clippy::from_over_into)]

extern crate chrono;
extern crate dotenv;
extern crate dotenv_codegen;
extern crate poise;
extern crate tokio;

mod commands;
mod embed;
mod prelude;

use commands::*;
use prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    println!("{}", dotenv!("DISCORD_TOKEN"));

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping()],
            ..Default::default()
        })
        .token(dotenv!("DISCORD_TOKEN"))
        .intents(GatewayIntents::all())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    bot_icon: ctx.cache.current_user().avatar_url().unwrap(),
                })
            })
        });

    framework.run().await?;

    Ok(())
}
