extern crate oreo_router;

use oreo_prelude::serenity::*;
use oreo_router::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoggingRequest {
    IsReady,

    LogInteractionCreate(Interaction),

    LogCategoryCreate(GuildChannel),
    LogCategoryUpdate(GuildChannel),
    LogCategoryDelete(GuildChannel),

    LogChannelCreate(GuildChannel),
    LogChannelUpdate(GuildChannel),
    LogChannelDelete(GuildChannel),

    LogMessageCreate(Message),
    LogMessageUpdate(MessageUpdateEvent),
    LogMessageDelete {
        guild_id: Option<GuildId>,
        channel_id: ChannelId,
        message_id: MessageId,
    },

    LogRoleCreate(Role),
    LogRoleUpdate(Role),
    LogRoleDelete {
        guild_id: GuildId,
        role_id: RoleId,
    },

    LogMemberJoin(Member),
    LogMemberUpdate(Member),
    LogMemberLeave {
        guild_id: GuildId,
        user: User,
    },

    LogReady,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoggingResponse {
    Ready,
    NotReady,
    Ok,
    Err(String),
}

impl Request for LoggingRequest {
    type Response = LoggingResponse;

    fn port() -> u16 {
        9000
    }
}
