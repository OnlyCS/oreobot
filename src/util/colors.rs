use crate::prelude::*;

pub const PRIMARY: (u8, u8, u8) = (43, 45, 49);
pub const WARN: (u8, u8, u8) = (178, 146, 3);
pub const ERROR: (u8, u8, u8) = (191, 36, 59);

pub fn hex_to_rgb<S>(hex: S) -> Result<(u8, u8, u8)>
where
    S: ToString,
{
    let hex_str = hex.to_string();
    let hex = hex_str.trim_start_matches('#');

    if hex.len() != 6 {
        bail!("Invalid hex color");
    }

    let r = u8::from_str_radix(&hex[0..2], 16)?;
    let g = u8::from_str_radix(&hex[2..4], 16)?;
    let b = u8::from_str_radix(&hex[4..6], 16)?;

    Ok((r, g, b))
}

pub fn hex_to_color<S>(hex: S) -> Result<serenity::Color>
where
    S: ToString,
{
    let (r, g, b) = hex_to_rgb(hex)?;

    Ok(serenity::Color::from_rgb(r, g, b))
}
