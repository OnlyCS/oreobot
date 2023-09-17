use poise::samples::HelpConfiguration;

use crate::prelude::*;

#[poise::command(track_edits, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), CommandError> {
    poise::builtins::help(
        ctx,
        command.as_ref().map(|n| n as &str),
        HelpConfiguration {
            ephemeral: true,
            show_context_menu_commands: false,
            ..Default::default()
        },
    )
    .await?;

    Ok(())
}
