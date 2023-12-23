pub(crate) use crate::{
    error::*,
    features::{clone::MessageCloneOptions, *},
    mpmc,
    util::{
        embed::{self, EmbedStatus},
        emoji,
        mention::{self, MentionType},
    },
};

pub use futures::stream::{self, StreamExt};
pub use oreo_prelude::{
    serenity::{
        ButtonStyle, CacheHttp, Channel, ChannelId, ChannelType, CreateActionRow, CreateButton,
        CreateEmbed, CreateEmbedFooter, Embed, ExecuteWebhook, FullEvent, Guild, GuildChannel,
        GuildId, Http, Member, Message, MessageId, Role, RoleId, User, UserId, Webhook,
    },
    *,
};
pub use oreo_router::*;
pub use rayon::prelude::*;
pub use std::{fmt, sync::Arc};
pub use thiserror::Error;
pub use tokio::sync::Mutex;

#[derive(Debug)]
pub struct SharedData {}

pub type Data = Arc<Mutex<SharedData>>;
pub type Context<'a> = poise::Context<'a, Data, CommandError>;
pub type FrameworkContext<'a> = poise::FrameworkContext<'a, Data, CommandError>;
