pub enum Color {
    Primary,
    Error,
    Success,
    Warning,
}

impl Into<(u8, u8, u8)> for Color {
    fn into(self) -> (u8, u8, u8) {
        match self {
            Color::Primary => (47, 49, 54),
            Color::Error => (140, 16, 31),
            Color::Success => (20, 160, 51),
            Color::Warning => (191, 173, 9),
        }
    }
}

impl From<Color> for Colour {
    fn from(color: Color) -> Self {
        let (r, g, b) = color.into();
        Colour::from_rgb(r, g, b)
    }
}

#[derive(Clone)]
pub struct Data {
    pub bot_icon: String,
}

pub type Error = anyhow::Error;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub use crate::embed::EmbedAdditions;
pub use anyhow::{bail, ensure, Context as Ctx, Result};
pub use dotenv_codegen::dotenv;
pub use poise::serenity_prelude::*;

pub fn now() -> String {
    chrono::Utc::now().to_rfc3339()
}
