pub use crate::error::*;
pub use oreo_prelude::*;
pub use thiserror::Error;

pub struct Data {}

pub type Context<'a> = poise::Context<'a, Data, CommandError>;
