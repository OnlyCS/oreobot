pub(crate) use crate::{
    error::*,
    events::{
        emitter::{EmitterEvent, EventEmitter},
        events, payloads,
    },
    features::starboard,
    nci,
    prisma::{
        self,
        prisma_client::{
            attachment, channel, channel_category, interaction, message, message_pin, role, user,
            ChannelType, InteractionType, PrismaClient,
        },
    },
    settings::{all as settings, Settings, UserSetting},
    util::{
        color::{consts as colors, Color},
        data,
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
    collections::HashMap,
    default::{self, Default},
    str::FromStr,
    sync::Arc,
    thread,
};

pub use async_trait::async_trait;
pub use futures::lock::Mutex;
pub use itertools::Itertools;
pub use log::{debug, error, info, trace, warn};
pub use poise::serenity_prelude as serenity;
pub use prisma_client_rust::{NewClientError, QueryError, RelationNotFetchedError};
pub use serde::{Deserialize, Serialize};
pub use simple_logger::SimpleLogger;
pub use thiserror::Error;

pub type Shared<T> = Arc<Mutex<T>>;

pub struct Data {
    pub emitter: EventEmitter,
    pub settings: Settings,
}

impl serenity::TypeMapKey for Data {
    type Value = Arc<Mutex<Self>>;
}

pub type Context<'a> = poise::Context<'a, Shared<Data>, CommandError>;

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

macro_rules! async_non_blocking {
	($a:block) => {
		tokio::task::spawn(async move $a)
	};
}

pub(crate) use async_non_blocking;
