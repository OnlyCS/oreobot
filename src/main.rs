#![feature(
    default_free_fn,
    let_chains,
    drain_filter,
    tuple_trait,
    unboxed_closures
)]

#[macro_use]
extern crate dotenv_codegen;
extern crate anyhow;
extern crate chrono;
extern crate dotenv;
extern crate futures;
extern crate log;
extern crate poise;
extern crate serde;
extern crate serde_json;
extern crate simple_logger;
extern crate tokio;

mod commands;
mod events;
mod logging;
mod nci;
mod prelude;
mod prisma;
mod util;

use futures::FutureExt;

use crate::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .init()?;

    let handler = poise::EventWrapper(|ctx, event| {
        Box::pin(async move {
            events::event_handler(ctx, event).await.unwrap();
        })
    });

    let prisma_client = Arc::new(Mutex::new(prisma::create().await?));
    let prisma_client_clone = Arc::clone(&prisma_client);

    let event_emitter = Arc::new(Mutex::new(EventEmitter::new()));
    let event_emitter_clone = Arc::clone(&event_emitter);

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::ping::ping()],
            on_error: |error| {
                async move {
                    events::error::handle(error).await.unwrap_or(()); // dont throw error to prevent loop
                }
                .boxed()
            },
            ..default()
        })
        .token(dotenv!("BOT_TOKEN"))
        .intents(serenity::GatewayIntents::all())
        .client_settings(|client| {
            client
                .event_handler(handler)
                .type_map_insert::<PrismaTypeKey>(prisma_client_clone)
                .type_map_insert::<EventEmitterTypeKey>(event_emitter_clone)
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                logging::register(ctx).await?;
                share::register(ctx).await?;

                Ok(Data {
                    prisma: prisma_client,
                    emitter: event_emitter,
                })
            })
        });

    framework.run().await?;

    Ok(())
}
