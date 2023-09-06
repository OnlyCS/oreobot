use crate::prelude::*;

pub fn default_role(member: &serenity::Member) -> serenity::EditRole {
    let mut role = serenity::EditRole::default();

    role.name(member.user.name.clone());
    role.colour(colors::PRIMARY.into());

    role
}
