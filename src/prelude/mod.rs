pub struct Data;

pub type Error = anyhow::Error;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub use anyhow::{bail, ensure, Context as Ctx, Result};
pub use dotenv_codegen::dotenv;
pub use poise::serenity_prelude as serenity;
