pub use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EmbedStatus {
    Sucess,
    Warning,
    Error,
}

pub fn default(ctx: &Context<'_>, status: EmbedStatus) -> serenity::CreateEmbed {
    let mut embed = serenity::CreateEmbed::default();

    embed.color(match status {
        EmbedStatus::Sucess => colors::PRIMARY,
        EmbedStatus::Warning => colors::WARN,
        EmbedStatus::Error => colors::ERROR,
    });

    embed.footer(|f| {
		f
			.text("OreoBot")
			.icon_url(
				ctx
					.serenity_context()
					.cache
					.current_user()
					.avatar_url()
					.unwrap_or(
						"https://cdn.discordapp.com/avatars/1025778682394058772/83ba9fd96ca2f8931e7c7e0b7d8d2431.webp?size=80".to_string()	
					)
			)
	});

    embed.timestamp(chrono::Local::now());

    embed
}

pub fn serenity_default(ctx: &serenity::Context, status: EmbedStatus) -> serenity::CreateEmbed {
    let mut embed = serenity::CreateEmbed::default();

    embed.color(match status {
        EmbedStatus::Sucess => colors::PRIMARY,
        EmbedStatus::Warning => colors::WARN,
        EmbedStatus::Error => colors::ERROR,
    });

    embed.footer(|f| {
		f
			.text("OreoBot")
			.icon_url(
				ctx
					.cache
					.current_user()
					.avatar_url()
					.unwrap_or(
						"https://cdn.discordapp.com/avatars/1025778682394058772/83ba9fd96ca2f8931e7c7e0b7d8d2431.webp?size=80".to_string()	
					)
			)
	});

    embed.timestamp(chrono::Local::now());

    embed
}
