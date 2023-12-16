use crate::prelude::{serenity::*, *};

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

impl PartialEq for LoggingResponse {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Ready, Self::Ready) => true,
            (Self::NotReady, Self::NotReady) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct LoggingServer;

impl ServerMetadata for LoggingServer {
    type Request = LoggingRequest;
    type Response = LoggingResponse;
    type Error = serde_error::Error;

    const HOST: &'static str = "logger";
    const PORT: u16 = 9000;

    const READY_REQUEST: Self::Request = LoggingRequest::IsReady;
    const READY_TRUE: Self::Response = LoggingResponse::Ready;
    const READY_FALSE: Self::Response = LoggingResponse::NotReady;
}
