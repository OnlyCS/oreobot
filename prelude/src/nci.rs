use serenity_rs::model::prelude::*;
use std::num::NonZeroU64;

// Constants for NCI Server
pub mod roles {
    use super::*;

    pub const OVERRIDES: RoleId = RoleId::new(878033546848108606);
    pub const MEMBERS: RoleId = RoleId::new(1016810972415008850);
    pub const BOTS: RoleId = RoleId::new(813138438013452348);
    pub const BOOSTER: RoleId = RoleId::new(1022189509363904716);
    pub const EVERYONE: RoleId = RoleId::new(803315311663251537);
    pub const SECRET: RoleId = RoleId::new(1153141223398330398);

    pub fn is_color_role(role: RoleId) -> bool {
        !matches!(
            role,
            OVERRIDES | MEMBERS | BOTS | BOOSTER | EVERYONE | SECRET
        )
    }

    pub fn can_log(role: impl Into<i64> + Copy) -> bool {
        !matches!(RoleId::new(role.into() as u64), EVERYONE | BOOSTER)
    }
}

pub mod channels {
    use super::*;

    pub const CHAT: ChannelId = ChannelId::new(1014256055330549842);
    pub const NEWS: ChannelId = ChannelId::new(997661924546322472);
    pub const STARRED: ChannelId = ChannelId::new(1016113247662919760);
}

pub const ID: GuildId = GuildId::new(803315311663251537);
