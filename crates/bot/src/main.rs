#![feature(
    let_chains,
    tuple_trait,
    unboxed_closures,
    extract_if,
    associated_type_defaults,
    error_generic_member_access,
    never_type,
    const_for
)]

#[macro_use]
extern crate dotenvy_macro;
extern crate async_channel;
extern crate automod;
extern crate dotenvy;
extern crate futures;
extern crate oreo_prelude;
extern crate poise;
extern crate thiserror;
extern crate tokio;

mod commands;
mod error;
mod features;
mod integrations;
mod mpmc;
mod prelude;
mod server;
mod util;

use error::BotServerError;
use prelude::*;

#[tokio::main]
async fn main() -> Result<!, BotServerError> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Warn)
        .with_threads(true)
        .with_module_level("oreo_bot", log::LevelFilter::Debug)
        .with_module_level("oreo_router", log::LevelFilter::Debug)
        .init()?;

    let framework = poise::FrameworkBuilder::default()
        .options(poise::FrameworkOptions {
            commands: commands::all(),
            event_handler: mpmc::event::handler,
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            let ctx = ctx.clone();

            Box::pin(async move {
                ctx.set_presence(
                    Some(serenity::ActivityData::playing("with Oppenheimer")),
                    serenity::OnlineStatus::Online,
                );

                poise::builtins::register_globally(&ctx, &framework.options().commands).await?;

                features::share::register().await;
                features::logger::register().await;
                features::clone::register().await;
                features::impersonate::register().await;

                #[cfg(not(feature = "smarty-integration"))]
                features::news_clone::register().await;

                #[cfg(feature = "smarty-integration")]
                integrations::smarty::register().await;

                server::run(ctx).await?;

                Ok(Arc::new(Mutex::new(SharedData {})))
            })
        })
        .build();

    let mut client =
        serenity::ClientBuilder::new(dotenv!("BOT_TOKEN"), serenity::GatewayIntents::all())
            .framework(framework)
            .await?;

    client.start().await?;

    panic!("framework run returned (heh?)")
}
