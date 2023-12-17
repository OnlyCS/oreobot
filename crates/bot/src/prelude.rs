pub(crate) use crate::{
    error::*,
    features::share,
    mpmc,
    util::{
        embed::{self, EmbedStatus},
        emoji, mention,
    },
};

pub use futures::stream::{self, StreamExt};
pub use oreo_prelude::{
    serenity::{
        Channel, ChannelId, ChannelType, FullEvent, Guild, GuildChannel, GuildId, Member, Message,
        MessageId, Role, RoleId, User, UserId,
    },
    *,
};
pub use oreo_router::*;
pub use std::sync::Arc;
pub use thiserror::Error;
pub use tokio::sync::Mutex;

pub struct SharedData {}

pub type Data = Arc<Mutex<SharedData>>;
pub type Context<'a> = poise::Context<'a, Data, CommandError>;
pub type FrameworkContext<'a> = poise::FrameworkContext<'a, Data, CommandError>;
