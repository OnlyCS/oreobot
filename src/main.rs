#![feature(default_free_fn, let_chains, drain_filter)]

#[macro_use]
extern crate dotenv_codegen;
extern crate anyhow;
extern crate dotenv;
extern crate log;
extern crate poise;
extern crate serde;
extern crate simple_logger;
extern crate tokio;

mod commands;
mod nci;
mod prelude;
mod prisma;
mod startup;

use crate::prelude::*;

#[tokio::main]
async fn main() {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .init()
        .unwrap();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::ping::ping()],
            ..default()
        })
        .token(dotenv!("BOT_TOKEN"))
        .intents(serenity::GatewayIntents::all())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                let mut prisma_client = prisma::create().await?;

                startup::syncdb(ctx, &mut prisma_client).await?;
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                Ok(Data {
                    prisma: prisma_client,
                })
            })
        });

    framework.run().await.unwrap();
}
