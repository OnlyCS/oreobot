use crate::prelude::*;

pub fn default_role(member: &serenity::Member) -> Result<serenity::EditRole> {
    let mut role = serenity::EditRole::default();

    role.name(member.user.name.clone());
    role.colour(colors::hex_to_color("FF0000")?.0.into());

    Ok(role)
}
