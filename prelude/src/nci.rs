use serenity_rs::model::prelude::*;
use std::num::NonZeroU64;

// Constants for NCI Server
pub mod roles {
    use super::*;

    pub const OVERRIDES: RoleId = RoleId(NonZeroU64::new(878033546848108606).unwrap());
    pub const MEMBERS: RoleId = RoleId(NonZeroU64::new(1016810972415008850).unwrap());
    pub const BOTS: RoleId = RoleId(NonZeroU64::new(813138438013452348).unwrap());
    pub const BOOSTER: RoleId = RoleId(NonZeroU64::new(1022189509363904716).unwrap());
    pub const EVERYONE: RoleId = RoleId(NonZeroU64::new(803315311663251537).unwrap());
    pub const SECRET: RoleId = RoleId(NonZeroU64::new(1153141223398330398).unwrap());

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

    pub const CHAT: ChannelId = ChannelId(NonZeroU64::new(1014256055330549842).unwrap());
    pub const NEWS: ChannelId = ChannelId(NonZeroU64::new(997661924546322472).unwrap());
    pub const STARRED: ChannelId = ChannelId(NonZeroU64::new(1016113247662919760).unwrap());
}

pub const ID: GuildId = GuildId(NonZeroU64::new(803315311663251537).unwrap());
