// yes to unstable rust, no to unsafe rust :)
#![feature(
    let_chains,
    tuple_trait,
    unboxed_closures,
    extract_if,
    async_fn_in_trait,
    associated_type_defaults,
    return_position_impl_trait_in_trait
)]

#[macro_use]
extern crate dotenv_codegen;
extern crate anyhow;
extern crate async_trait;
extern crate chrono;
extern crate color_name;
extern crate dotenv;
extern crate futures;
extern crate itertools;
extern crate log;
extern crate poise;
extern crate serde;
extern crate serde_json;
extern crate simple_logger;
extern crate tokio;

mod cache;
mod commands;
mod error;
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
async fn main() -> Result<(), AnyError> {
    SimpleLogger::new()
        .with_level(if cfg!(debug_assertions) {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .init()
        .make_error(anyhow!("failed to initialize logger"))?;

    let handler = poise::EventWrapper(|ctx, event| {
        Box::pin(async move {
            events::event_handler(ctx, event).await.unwrap();
        })
    });

    let data = Arc::new(Mutex::new(Data {
        emitter: EventEmitter::new(),
        cache: Cache::new(),
    }));

    let data_serenity = Arc::clone(&data);

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::ping::ping(),
                commands::setting::settings(),
                commands::impersonate::impersonate(),
            ],
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
                .type_map_insert::<Data>(data_serenity)
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                let ctx = ctx.clone();
                logging::register(&ctx).await;

                ctx.set_presence(
                    Some(serenity::Activity::playing("with Oppenheimer")),
                    serenity::OnlineStatus::Online,
                )
                .await;

                poise::builtins::register_globally(&ctx, &framework.options().commands).await?;

                async_non_blocking!({
                    impersonate::register(&ctx).await;
                    share::register(&ctx).await;
                    starboard::register(&ctx).await.unwrap();
                    clone::register(&ctx).await.unwrap();
                });

                Ok(data)
            })
        });

    framework.run().await?;

    Ok(())
}
