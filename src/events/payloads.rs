use crate::prelude::{serenity::*, *};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageDeletePayload {
    pub guild_id: Option<GuildId>,
    pub channel_id: ChannelId,
    pub message_id: MessageId,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoleDeletePayload {
    pub guild_id: GuildId,
    pub role_id: RoleId,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MemberLeavePayload {
    pub guild_id: GuildId,
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageReactionAddPayload {
    pub reaction: Reaction,
    pub message: Message,
}
