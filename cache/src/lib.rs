extern crate oreo_prelude;
extern crate oreo_proc_macros;
extern crate oreo_router;
extern crate serde;

use oreo_prelude::{serenity::*, *};
use oreo_proc_macros::update_enum;
use oreo_router::Request;
use serde::{Deserialize, Serialize};

#[update_enum]
pub struct UserSettings {
    pin_confirm: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CacheRequest {
    GetRoleColor(UserId),
    GetRoleName(UserId),
    SetRoleColor(UserId, Color),
    SetRoleName(UserId, String),

    GetCustomRoles,
    AddCustomRole(RoleId),
    RemoveCustomRole(RoleId),

    GetImpersonation(UserId),
    SetImpersonation(UserId, UserId),
    StopImpersonation(UserId),

    GetNewsInChatMessage(MessageId),
    AddNewsInChatMessage {
        message_news: Message,
        message_chat: MessageId,
    },

    GetUserSettings(UserId),
    UpdateUserSetting {
        user_id: UserId,
        setting: UpdateUserSettings,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CacheResponse {
    Ready,
    NotReady,
    SetOk,
    Err(String),

    RoleColorOk(Color),
    RoleNameOk(String),
    CustomRolesOk(Vec<RoleId>),
    ImpersonationOk(Option<UserId>),
    NewsInChatMessageOk(Option<Message>),
    UserSettingsOk(UserSettings),
}

impl Request for CacheRequest {
    type Response = CacheResponse;

    fn port() -> u16 {
        9001
    }
}
