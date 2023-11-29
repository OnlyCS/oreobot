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
extern crate dotenvy;
extern crate futures;
extern crate oreo_prelude;
extern crate poise;
extern crate thiserror;
extern crate tokio;

mod commands;
mod error;
mod features;
mod prelude;
mod util;

use error::BotServerError;
use prelude::*;

#[tokio::main]
async fn main() -> Result<!, BotServerError> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()?;

    let framework = poise::FrameworkBuilder::default()
        .options(poise::FrameworkOptions {
            commands: vec![commands::ping::ping()],
            ..Default::default()
        })
        .token(dotenv!("BOT_TOKEN"))
        .intents(serenity::GatewayIntents::all())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                ctx.set_presence(
                    Some(serenity::ActivityData::playing("with Oppenheimer")),
                    serenity::OnlineStatus::Online,
                );

                poise::builtins::register_globally(&ctx, &framework.options().commands).await?;

                Ok(Data {})
            })
        });

    framework.run().await?;

    panic!("framework run returned (heh?)")
}
