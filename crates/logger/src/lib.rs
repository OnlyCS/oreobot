#![feature(trace_macros)]

extern crate oreo_prelude;
extern crate oreo_router;
extern crate serde;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use oreo_prelude::serenity::*;
use oreo_prelude::*;
use oreo_router::Request;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoggingRequest {
    IsReady,

    InteractionCreate(Interaction),
    InteractionRead(InteractionId),

    CategoryCreate(GuildChannel),
    CategoryRead(ChannelId),
    CategoryUpdate(GuildChannel),
    CategoryDelete(ChannelId),

    ChannelCreate(GuildChannel),
    ChannelRead(ChannelId),
    ChannelUpdate(GuildChannel),
    ChannelDelete(ChannelId),

    MessageCreate(Message),
    MessageRead(MessageId),
    MessageUpdate(MessageUpdateEvent),
    MessageDelete(MessageId),

    RoleCreate(Role),
    RoleSetBlacklisted(RoleId),
    RoleRead(RoleId),
    RoleReadAll,
    RoleUpdate(Role),
    RoleDelete(RoleId),

    MemberCreate(Member),
    MemberRead(UserId),
    MemberUpdate(Member),
    MemberDelete(UserId),

    UserSettingsCreate(UserId, UserSettings),
    UserSettingsRead(UserId),
    UserSettingsReadAll,
    UserSettingsUpdate(UserId, UpdateUserSettings),

    MessageCloneCreate {
        source: serenity::MessageId,
        clone: serenity::MessageId,
        destination: serenity::ChannelId,
        reason: MessageCloneReason,
        update: bool,
        update_delete: bool,
    },

    LoggerReady,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoggingResponse {
    Ready,
    NotReady,
    UpdateOk,
    Err(String),

    AllRolesOk(HashMap<RoleId, prisma::data::RoleData>),
    AllUserSettingsOk(HashMap<UserId, UserSettings>),
    AllNewsInChatOk(HashMap<MessageId, MessageId>),

    MemberOk(prisma::data::UserData),
    RoleOk(prisma::data::RoleData),
    MessageOk(prisma::data::MessageData),
    ChannelOk(prisma::data::ChannelData),
    CategoryOk(prisma::data::ChannelCategoryData),
    InteractionOk(prisma::data::InteractionData),
    NewsInChatOk(MessageId, MessageId),
    UserSettingsOk(UserSettings),
}

impl Request for LoggingRequest {
    type Response = LoggingResponse;

    const PORT: u16 = 9000;
    const HOST: &'static str = "logger";
}
