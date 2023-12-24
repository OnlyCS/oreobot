use serenity_rs::model::prelude::*;

// Constants for NCI Server
pub mod roles {
    use super::*;

    pub const OVERRIDES: RoleId = RoleId::new(878033546848108606);
    pub const MEMBERS: RoleId = RoleId::new(1016810972415008850);
    pub const BOTS: RoleId = RoleId::new(813138438013452348);
    pub const BOOSTER: RoleId = RoleId::new(1022189509363904716);
    pub const EVERYONE: RoleId = RoleId::new(803315311663251537);
}

pub mod channels {
    use super::*;

    pub const CHAT: ChannelId = ChannelId::new(1014256055330549842);
    pub const NEWS: ChannelId = ChannelId::new(997661924546322472);
    pub const STARRED: ChannelId = ChannelId::new(1016113247662919760);
    pub const LOGGING: ChannelId = ChannelId::new(1188138920937013319);
}

pub mod webhook {
    pub const NAME: &'static str = "Oreo v2's Internals";
}

pub mod smarty {
    use super::*;

    pub const ID: UserId = UserId::new(809111302198001724);
    pub const WEBHOOK_CHAT: WebhookId = WebhookId::new(1142657223017910394);
}

pub const ID: GuildId = GuildId::new(803315311663251537);
