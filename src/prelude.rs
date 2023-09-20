#[allow(unused_imports)]
pub(crate) use crate::{
    cache::{self, all as cache_items, Cache},
    error::*,
    events::{
        emitter::{EmitterEvent, EventEmitter},
        events, payloads,
    },
    features::{impersonate, newsinchat, starboard},
    nci,
    prisma::{
        self,
        prisma_client::{
            attachment, channel, channel_category, interaction, message, message_pin, news_in_chat,
            role, user, ChannelType, InteractionType, PrismaClient,
        },
    },
    util::{
        color::{consts as colors, Color},
        data,
        embed::{self, EmbedStatus},
        ephemeral, is_admin, latency,
        loading::{
            Loading, WithInteraction as LoadingWithInteraction,
            WithoutInteraction as LoadingWithoutInteraction,
        },
        message::{
            clone, emoji,
            mention::{self, MentionType},
        },
        role::default_role,
        share,
        string::StringUtil,
    },
};

pub use std::{
    collections::HashMap,
    default::{self, Default},
    num,
    str::FromStr,
    sync::Arc,
    thread,
};

pub use async_trait::async_trait;
pub use futures::lock::Mutex;
pub use itertools::Itertools;
pub use log::{debug, error, info, trace, warn};
pub use poise::serenity_prelude as serenity;
pub use prisma_client_rust::{and, not, or, NewClientError, QueryError, RelationNotFetchedError};
pub use rand::prelude::*;
pub use serde::{Deserialize, Serialize};
pub use simple_logger::SimpleLogger;
pub use thiserror::Error;

pub type Shared<T> = Arc<Mutex<T>>;

pub struct Data {
    pub emitter: EventEmitter,
    pub cache: Cache,
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

macro_rules! async_non_blocking {
	($a:block) => {
		tokio::task::spawn(async move $a)
	};
}

pub(crate) use async_non_blocking;
