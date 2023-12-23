mod color;
mod custom;

use crate::prelude::*;

#[poise::command(slash_command, subcommands("color::color", "custom::custom"))]
pub async fn role(_: Context<'_>) -> Result<(), CommandError> {
    Ok(())
}
