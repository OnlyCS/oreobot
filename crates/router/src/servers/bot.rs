use crate::prelude::{serenity::*, *};

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum BotServerError {
    #[error("Serenity error: {0}")]
    Serenity(String),

    #[error("Error with logging server: {0}")]
    LoggingServer(String),
}

impl From<serenity::Error> for BotServerError {
    fn from(value: serenity::Error) -> Self {
        Self::Serenity(value.to_string())
    }
}

impl From<RouterError<crate::LoggingServer>> for BotServerError {
    fn from(value: RouterError<crate::LoggingServer>) -> Self {
        Self::LoggingServer(value.to_string())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BotRequest {
    IsReady,

    CreateColorRole { user_id: UserId, custom_roles: u16 },

    AddRoleToUser(UserId, RoleId),
    GetRolesOfUser(UserId),
    RemoveRoleFromUser(UserId, RoleId),
    DeleteRole(RoleId),
    GetMember(UserId),

    GetAllRoles,
    GetAllMembers,
    GetAllCategories,
    GetAllChannels,

    UserExists(UserId),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BotResponse {
    Ready,
    NotReady,

    CreateRoleOk(Role),
    MemberOk(Member),

    RolesOk(Vec<Role>),
    MembersOk(Vec<Member>),
    CategoriesOk(Vec<GuildChannel>),
    ChannelsOk(Vec<GuildChannel>),

    UserExistsOk(bool),

    UpdateOk,
}

impl PartialEq for BotResponse {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Ready, Self::Ready) => true,
            (Self::NotReady, Self::NotReady) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct BotServer;

impl ServerMetadata for BotServer {
    type Request = BotRequest;
    type Response = BotResponse;
    type Error = BotServerError;

    const HOST: &'static str = "bot";
    const PORT: u16 = 9002;

    const READY_REQUEST: Self::Request = BotRequest::IsReady;
    const READY_TRUE: Self::Response = BotResponse::Ready;
    const READY_FALSE: Self::Response = BotResponse::NotReady;
}
