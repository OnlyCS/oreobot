pub(crate) use crate::nci; // needs pub(crate) but idk why
pub(crate) use crate::prisma;
pub use crate::prisma::prisma_client::{
    attachment, channel, channel_category, message, user, user_role, ChannelType as PChannelType,
};
pub use crate::prisma::PrismaClient;
pub use anyhow::{bail, Context as ToAnyhowResult, Result};
pub use log::{debug, error, info, trace, warn};
pub use poise::serenity_prelude as serenity;
pub use simple_logger::SimpleLogger;
pub use std::default::default;

pub struct Data {
    pub prisma: PrismaClient,
}

pub type Context<'a> = poise::Context<'a, Data, anyhow::Error>;

pub trait IsThread {
    fn is_thread(&self) -> bool;
}

impl IsThread for serenity::GuildChannel {
    fn is_thread(&self) -> bool {
        self.thread_metadata.is_some()
    }
}
