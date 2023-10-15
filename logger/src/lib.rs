extern crate oreo_prelude;
extern crate oreo_router;
extern crate serde;

use oreo_prelude::prisma;
use oreo_prelude::serenity::*;
use oreo_router::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoggingRequest {
    IsReady,

    LogInteractionCreate(Interaction),
    GetInteraction(InteractionId),

    LogCategoryCreate(GuildChannel),
    LogCategoryUpdate(GuildChannel),
    LogCategoryDelete(GuildChannel),
    GetCategory(ChannelId),

    LogChannelCreate(GuildChannel),
    LogChannelUpdate(GuildChannel),
    LogChannelDelete(GuildChannel),
    GetChannel(ChannelId),

    LogMessageCreate(Message),
    LogMessageUpdate(MessageUpdateEvent),
    LogMessageDelete {
        guild_id: Option<GuildId>,
        channel_id: ChannelId,
        message_id: MessageId,
    },
    GetMessage(MessageId),

    LogRoleCreate(Role),
    LogRoleUpdate(Role),
    LogRoleDelete {
        guild_id: GuildId,
        role_id: RoleId,
    },
    GetRole(RoleId),

    LogMemberJoin(Member),
    LogMemberUpdate(Member),
    LogMemberLeave {
        guild_id: GuildId,
        user: User,
    },
    GetMember(UserId),

    LogReady,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoggingResponse {
    Ready,
    NotReady,
    UpdateOk,
    Err(String),

    MemberOk(prisma::data::UserData),
    RoleOk(prisma::data::RoleData),
    MessageOk(prisma::data::MessageData),
    ChannelOk(prisma::data::ChannelData),
    CategoryOk(prisma::data::ChannelCategoryData),
    InteractionOk(prisma::data::InteractionData),
}

impl Request for LoggingRequest {
    type Response = LoggingResponse;

    fn port() -> u16 {
        9000
    }
}
