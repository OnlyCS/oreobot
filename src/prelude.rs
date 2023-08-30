pub(crate) use crate::{
    events::{
        emitter::{EmitterEvent, EventEmitter, EventEmitterTypeKey},
        events, payloads,
    },
    features::starboard,
    logging, nci,
    prisma::{
        self,
        prisma_client::{
            attachment, channel, channel_category, interaction, message, message_pin, role, user,
            ChannelType, InteractionType, PrismaClient,
        },
    },
    util::{
        colors,
        embed::{self, EmbedStatus},
        is_admin, latency,
        loading::Loading,
        message::{
            clone, emoji,
            mention::{self, MentionType},
        },
        role::default_role,
        share,
    },
};

pub use std::{
    default::{self, Default},
    str::FromStr,
    sync::Arc,
    thread,
};

pub use anyhow::{anyhow, bail, Context as ToAnyhowResult, Result};
pub use log::{debug, error, info, trace, warn};
pub use serde::{Deserialize, Serialize};

pub use futures::lock::Mutex;
pub use poise::serenity_prelude as serenity;
pub use simple_logger::SimpleLogger;

pub use itertools::Itertools;

pub type Shared<T> = Arc<Mutex<T>>;

#[derive(Debug)]
pub struct Data {
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
