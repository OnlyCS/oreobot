#![feature(let_chains, error_generic_member_access, never_type)]

#[macro_use]
extern crate dotenvy_macro;
extern crate async_channel;
extern crate automod;
extern crate dotenvy;
extern crate futures;
extern crate oreo_prelude;
extern crate poise;
extern crate serde_json;
extern crate thiserror;
extern crate tokio;

mod commands;
mod error;
mod features;
mod mpmc;
mod prelude;
mod server;
mod util;

use prelude::*;

#[tokio::main]
async fn main() -> Result<!, BotError> {
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
            on_error: |err| {
                Box::pin(async move {
                    error!("Got poise error: {}", err);

                    if let Err(handle_err) = match err {
                        poise::FrameworkError::Command { error, ctx, .. } => {
                            let reply = poise::CreateReply::default()
                                .ephemeral(true)
                                .components(vec![share::row()])
                                .embed(error::build_embed(&error));

                            /* goofy unwrap to prevent on_error from being called forever */
                            ctx.send(reply).await.map(|_| ())
                        }
                        _ => poise::builtins::on_error(err).await,
                    } {
                        error!("Error while handling error: {}", handle_err);
                    }
                })
            },
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

                // features::share::register().await;
                // features::logger::register().await;
                // features::clone::register().await;
                features::impersonate::register().await;
                // features::starboard::register().await;
                // features::news_clone::register().await;
                feature::antithumbs::register().await;

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
