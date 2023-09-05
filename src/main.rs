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

mod commands;
mod events;
mod features;
mod logging;
mod nci;
mod prelude;
mod prisma;
mod settings;
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

    let data = Arc::new(Mutex::new(Data {
        emitter: EventEmitter::new(),
        settings: Settings::new(),
    }));

    let data_serenity = Arc::clone(&data);

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::ping::ping(), commands::setting::settings()],
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
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                logging::register(ctx).await?;
                share::register(ctx).await?;
                starboard::register(ctx).await?;
                clone::register(ctx).await?;

                Ok(data)
            })
        });

    framework.run().await?;

    Ok(())
}
