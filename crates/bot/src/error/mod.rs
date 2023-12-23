mod errors;

use crate::prelude::*;
pub use errors::*;

pub fn build_embed(error: &CommandError) -> serenity::CreateEmbed {
    embed::default(EmbedStatus::Error).title("Oreo2 | Error").description(format!(
		"An error occurred:\n{error}\nThis error has been handled and the bot will continue to function normally."
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
