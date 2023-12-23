use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EmbedStatus {
    Success,
    Warning,
    Error,
}

impl From<EmbedStatus> for Color {
    fn from(value: EmbedStatus) -> Self {
        match value {
            EmbedStatus::Success => colors::PRIMARY,
            EmbedStatus::Warning => colors::WARN,
            EmbedStatus::Error => colors::ERROR,
        }
    }
}

pub fn default(status: EmbedStatus) -> CreateEmbed {
    CreateEmbed::new()
        .color(Color::from(status))
        .footer(CreateEmbedFooter::new("Oreo").icon_url("https://cdn.discordapp.com/avatars/1025778682394058772/83ba9fd96ca2f8931e7c7e0b7d8d2431.webp?size=80"))
        .timestamp(chrono::Local::now())
}
