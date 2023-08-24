#![feature(let_chains, tuple_trait, unboxed_closures, extract_if)]

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
mod features;
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
        .with_level(log::LevelFilter::Info)
        .init()?;

    let handler = poise::EventWrapper(|ctx, event| {
        Box::pin(async move {
            events::event_handler(ctx, event).await.unwrap();
        })
    });

    let event_emitter = Arc::new(Mutex::new(EventEmitter::new()));
    let event_emitter_clone = Arc::clone(&event_emitter);

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::ping::ping(), commands::role::role()],
            on_error: |error| {
                async move {
                    events::error::handle(error).await.unwrap_or(()); // dont throw error to prevent loop
                }
                .boxed()
            },
            ..Default::default()
        })
        .token(dotenv!("BOT_TOKEN"))
        .intents(serenity::GatewayIntents::all())
        .client_settings(|client| {
            client
                .event_handler(handler)
                .type_map_insert::<EventEmitterTypeKey>(event_emitter_clone)
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                logging::register(ctx).await?;
                share::register(ctx).await?;
                starboard::register(ctx).await?;
                clone::register(ctx).await?;

                Ok(Data {
                    emitter: event_emitter,
                })
            })
        });

    framework.run().await?;

    Ok(())
}
