use crate::prelude::{serenity::*, *};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComponentInteractionPayload(pub MessageComponentInteraction);

impl From<MessageComponentInteraction> for ComponentInteractionPayload {
    fn from(value: MessageComponentInteraction) -> Self {
        Self(value)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnyInteractionPayload(pub Interaction);

impl From<Interaction> for AnyInteractionPayload {
    fn from(value: Interaction) -> Self {
        Self(value)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageCreatePayload(pub Message);

impl From<Message> for MessageCreatePayload {
    fn from(value: Message) -> Self {
        Self(value)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageUpdatePayload(pub MessageUpdateEvent);

impl From<MessageUpdateEvent> for MessageUpdatePayload {
    fn from(value: MessageUpdateEvent) -> Self {
        Self(value)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageDeletePayload {
    pub guild_id: Option<GuildId>,
    pub channel_id: ChannelId,
    pub message_id: MessageId,
}

impl From<(Option<GuildId>, ChannelId, MessageId)> for MessageDeletePayload {
    fn from(value: (Option<GuildId>, ChannelId, MessageId)) -> Self {
        Self {
            guild_id: value.0,
            channel_id: value.1,
            message_id: value.2,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelCreatePayload(pub GuildChannel);

impl From<GuildChannel> for ChannelCreatePayload {
    fn from(value: GuildChannel) -> Self {
        Self(value)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelUpdatePayload(pub GuildChannel);

impl From<GuildChannel> for ChannelUpdatePayload {
    fn from(value: GuildChannel) -> Self {
        Self(value)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelDeletePayload(pub GuildChannel);

impl From<GuildChannel> for ChannelDeletePayload {
    fn from(value: GuildChannel) -> Self {
        Self(value)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryCreatePayload(pub ChannelCategory);

impl From<ChannelCategory> for CategoryCreatePayload {
    fn from(value: ChannelCategory) -> Self {
        Self(value)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryUpdatePayload(pub ChannelCategory);

impl From<ChannelCategory> for CategoryUpdatePayload {
    fn from(value: ChannelCategory) -> Self {
        Self(value)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryDeletePayload(pub ChannelCategory);

impl From<ChannelCategory> for CategoryDeletePayload {
    fn from(value: ChannelCategory) -> Self {
        Self(value)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoleCreatePayload(pub Role);

impl From<Role> for RoleCreatePayload {
    fn from(value: Role) -> Self {
        Self(value)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoleUpdatePayload(pub Role);

impl From<Role> for RoleUpdatePayload {
    fn from(value: Role) -> Self {
        Self(value)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoleDeletePayload {
    pub guild_id: GuildId,
    pub role_id: RoleId,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReadyEventPayload(pub Ready);

impl From<Ready> for ReadyEventPayload {
    fn from(value: Ready) -> Self {
        Self(value)
    }
}
