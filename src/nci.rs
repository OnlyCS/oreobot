use crate::prelude::serenity::*;

// Constants for NCI Server
pub mod roles {
    use crate::prelude::serenity::*;

    pub const OVERRIDES: RoleId = RoleId(878033546848108606);
    pub const MEMBERS: RoleId = RoleId(1016810972415008850);
    pub const BOTS: RoleId = RoleId(813138438013452348);
}

pub mod channels {
    use crate::prelude::serenity::*;

    pub const CHAT: ChannelId = ChannelId(1014256055330549842);
    pub const NEWS: ChannelId = ChannelId(997661924546322472);
    pub const STARRED: ChannelId = ChannelId(1016113247662919760);
}

pub mod smarty {
    use crate::prelude::serenity::*;

    pub const CHAT_WH: WebhookId = WebhookId(1013868027500052561);
}

pub const ID: GuildId = GuildId(803315311663251537);
