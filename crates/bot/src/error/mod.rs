mod errors;

use crate::prelude::*;
pub use errors::*;

pub fn reformat_error(error: String) -> String {
    let split = error
        .split(":")
        .map(|s| s.trim())
        .enumerate()
        .map(|(idx, s)| format!("{}{}", " ".repeat(idx), s))
        .join(":\n");

    format!("```\n{}\n```", split)
}

pub fn build_embed(error: &CommandError) -> serenity::CreateEmbed {
    let err_str = error.to_string();
    let is_warning = err_str.to_lowercase().contains("warning");

    embed::default(if is_warning {
        EmbedStatus::Warning
    } else {
        EmbedStatus::Error
    })
    .title("Oreo2 | Error")
    .description(format!(
		"An error occurred:\n{}\nThis {} has been handled and the bot will continue to function normally.",
		reformat_error(err_str),
		if is_warning { "warning" } else { "error" },
	))
}

pub async fn handle(
    ctx: impl serenity::CacheHttp,
    error: CommandError,
    log_in: ChannelId,
) -> Result<(), serenity::Error> {
    let message = serenity::CreateMessage::new().add_embed(build_embed(&error));

    log_in.send_message(&ctx, message).await?;

    Ok(())
}
