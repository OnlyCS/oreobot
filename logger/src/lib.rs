#![feature(trace_macros)]

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
    RoleRead(RoleId),
    RoleUpdate(Role),
    RoleDelete(RoleId),

    MemberCreate(Member),
    MemberRead(UserId),
    MemberUpdate(Member),
    MemberDelete(UserId),

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
