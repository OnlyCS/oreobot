use crate::{bail, error::*, serenity, Itertools};

use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ColorParseError {
    #[error("failed to parse from hex {0}")]
    ParseHex(String),

    #[error("failed to parse from color name {0}")]
    ParseName(String),

    #[error("Color::from_str failed to parse {0}")]
    ParseStr(String),
}

pub mod consts {
    use super::*;

    pub const PRIMARY: Color = Color {
        r: 43,
        g: 45,
        b: 49,
    };

    pub const WARN: Color = Color {
        r: 178,
        g: 146,
        b: 3,
    };

    pub const ERROR: Color = Color {
        r: 191,
        g: 36,
        b: 59,
    };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn from_hex<S>(hex: S) -> Result<Self, ColorParseError>
    where
        S: ToString,
    {
        let (r, g, b) = hex
            .to_string()
            .trim_start_matches("#") // remove preceding #
            .chars() // iterate
            .chunks(2) // chunks of two because thats how hex works
            .into_iter()
            .map(|n| n.collect::<String>()) // make each group a string
            .map(|n| u8::from_str_radix(&n, 16))
            .map(|n| n.make_error(ColorParseError::ParseHex(hex.to_string())))
            .collect_tuple() // r,g,b
            .make_error(ColorParseError::ParseHex(hex.to_string()))?;

        Ok(Self {
            r: r?,
            g: g?,
            b: b?,
        })
    }

    /// Converts into format: #ffffff
    pub fn to_hex(&self) -> String {
        format!("#{}", self.to_raw_hex())
    }

    /// Converts into format: ffffff (no #)
    pub fn to_raw_hex(&self) -> String {
        vec![self.r, self.g, self.b]
            .into_iter()
            .map(|n| format!("{:x}", n))
            .join("")
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn from_color_name<S>(name: S) -> Result<Self, ColorParseError>
    where
        S: ToString,
    {
        let (r, g, b) = color_name::Color::val()
            .by_string(name.to_string())
            .make_error(ColorParseError::ParseName(name.to_string()))?
            .into_iter()
            .collect_tuple()
            .make_error(ColorParseError::ParseName(name.to_string()))?;

        Ok(Self { r, g, b })
    }

    pub fn database(this: impl Into<Self>) -> String {
        this.into().to_raw_hex()
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from(value: (u8, u8, u8)) -> Self {
        Self {
            r: value.0,
            g: value.1,
            b: value.2,
        }
    }
}

impl From<Color> for (u8, u8, u8) {
    fn from(value: Color) -> Self {
        (value.r, value.g, value.b)
    }
}

impl From<Color> for serenity::Color {
    fn from(value: Color) -> Self {
        serenity::Color::from_rgb(value.r, value.g, value.b)
    }
}

impl From<serenity::Color> for Color {
    fn from(value: serenity::Color) -> Self {
        Self::from((value.r(), value.g(), value.b()))
    }
}

impl TryFrom<String> for Color {
    type Error = ColorParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(&value)
    }
}

impl TryFrom<&String> for Color {
    type Error = ColorParseError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl FromStr for Color {
    type Err = ColorParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let value = value.to_string();

        let from_hex = Color::from_hex(&value);
        let from_rgb: Result<Color, ColorParseError> = {
            let to_parse = value.trim_start_matches("(").trim_end_matches(")");
            let results = to_parse.split(",").map(str::trim).map(u8::from_str);

            if results.clone().any(|n| n.is_err()) {
                Err(ColorParseError::ParseStr(value.to_string()))
            } else {
                Ok(results
                    .map(Result::unwrap)
                    .collect_tuple::<(u8, u8, u8)>()
                    .map(Color::from)
                    .unwrap())
            }
        };
        let from_name = Color::from_color_name(&value);

        vec![from_hex, from_rgb, from_name]
            .into_iter()
            .filter_map(Result::ok)
            .next()
            .make_error(ColorParseError::ParseStr(String::from(value)))
    }
}

impl From<Color> for u32 {
    fn from(value: Color) -> Self {
        serenity::Color::from(value).0
    }
}

impl From<Color> for u64 {
    fn from(value: Color) -> Self {
        u32::from(value).into()
    }
}

#[cfg(feature = "color-arg")]
#[async_trait::async_trait]
impl poise::SlashArgument for Color {
    fn create(builder: serenity::CreateCommandOption) -> serenity::CreateCommandOption {
        builder.kind(serenity::CommandOptionType::String)
    }

    async fn extract(
        _ctx: &serenity::Context,
        _interaction: &serenity::CommandInteraction,
        value: &serenity::ResolvedValue<'_>,
    ) -> Result<Self, poise::SlashArgError> {
        let serenity::ResolvedValue::String(str) = value else {
            bail!(poise::SlashArgError::new_command_structure_mismatch(
                "expected string"
            ))
        };

        let color = Color::from_str(str)
            .map_err(|_| poise::SlashArgError::new_command_structure_mismatch("invalid color"))?;

        Ok(color)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.to_hex())
    }
}
