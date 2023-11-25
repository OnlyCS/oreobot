#![feature(
    let_chains,
    tuple_trait,
    unboxed_closures,
    extract_if,
    associated_type_defaults,
    error_generic_member_access
)]

#[macro_use]
extern crate dotenvy_macro;
extern crate dotenvy;
extern crate oreo_prelude;
extern crate poise;
extern crate thiserror;
extern crate tokio;

mod error;
mod features;
mod prelude;

use error::BotServerError;
use prelude::*;

#[tokio::main]
async fn main() -> Result<!, BotServerError> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()?;

    let framework = poise::FrameworkBuilder::default()
        .options(poise::FrameworkOptions {
			commands: vec![
				
			],
            ..Default::default()
        })
        .token(dotenv!("BOT_TOKEN"))
        .intents(serenity::GatewayIntents::all())
        .setup(|ctx, _ready, framework| Box::pin(async move { Ok(Data {}) }));

    framework.run().await?;

    Ok(panic!("framework run returned"))
}
