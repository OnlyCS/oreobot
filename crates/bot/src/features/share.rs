use crate::prelude::*;

pub fn button() -> serenity::CreateButton {
    serenity::CreateButton::new("Share")
        .custom_id("oreo2_share")
        .style(serenity::ButtonStyle::Secondary)
}

pub fn row() -> serenity::CreateActionRow {
    serenity::CreateActionRow::Buttons(vec![button()])
}
