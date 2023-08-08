pub(crate) use crate::{
    events::{
        emitter::{EmitterEvent, EventEmitter, EventEmitterTypeKey},
        event, payload,
    },
    nci,
    prisma::{
        self,
        prisma_client::{
            attachment, channel, channel_category, interaction, message, role, user, ChannelType,
            InteractionType, PrismaClient,
        },
        PrismaTypeKey,
    },
    util::{
        colors as color, embed, get_prisma, latency,
        message::{clone, emoji, mention},
        share,
    },
};

pub use std::{
    default::{self, Default},
    sync::Arc,
    thread,
};

pub use anyhow::{bail, Context as ToAnyhowResult, Result};
pub use log::{debug, error, info, trace, warn};
pub use serde::{Deserialize, Serialize};

pub use futures::lock::Mutex;
pub use poise::serenity_prelude as serenity;
pub use simple_logger::SimpleLogger;

pub type Shared<T> = Arc<Mutex<T>>;

#[derive(Debug)]
pub struct Data {
    pub prisma: Shared<PrismaClient>,
    pub emitter: Shared<EventEmitter>,
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

pub trait CapitalizeFirstLetter {
    fn capitalize_first_letter(&self) -> String;
}

impl<T> CapitalizeFirstLetter for T
where
    T: ToString,
{
    fn capitalize_first_letter(&self) -> String {
        let string = self.to_string();
        let mut chars = string.chars();
        match chars.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }
}
