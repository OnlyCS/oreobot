use std::fmt;

use crate::prelude::*;

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
    pub fn from_hex<S>(hex: S) -> Result<Self>
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
            .map(|n| u8::from_str_radix(&n, 16)) // parse to int
            .map(Result::unwrap) // unwrap like this because it looks cool
            .collect_tuple() // r,g,b
            .context("Problem parsing hex")?;

        Ok(Self { r, g, b })
    }

    pub fn into_hex(&self) -> String {
        format!("#{}", self.into_hex_no_hash())
    }

    pub fn into_hex_no_hash(&self) -> String {
        vec![self.r, self.g, self.b]
            .into_iter()
            .enumerate()
            .map(|(idx, n)| format!("{:x}", n))
            .join("")
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn from_color_name<S>(name: S) -> Result<Self>
    where
        S: ToString,
    {
        let (r, g, b) = color_name::Color::val()
            .by_string(name.to_string())
            .unwrap()
            .into_iter()
            .collect_tuple()
            .unwrap();

        Ok(Self { r, g, b })
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
        serenity::Colour::from_rgb(value.r, value.g, value.b)
    }
}

impl FromStr for Color {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self> {
        let value = value.to_string();

        let from_hex = Color::from_hex(&value);
        let from_rgb = {
            let to_parse = value.trim_start_matches("(").trim_end_matches(")");
            let results = to_parse.split(",").map(str::trim).map(u8::from_str);

            if results.clone().any(|n| n.is_err()) {
                Err(anyhow!("Could not parse from rgb"))
            } else {
                Ok(results
                    .map(Result::unwrap)
                    .collect_tuple::<(u8, u8, u8)>()
                    .map(Color::from)
                    .unwrap())
            }
        };
        let from_name = Color::from_color_name(value);

        vec![from_hex, from_rgb, from_name]
            .into_iter()
            .filter_map(Result::ok)
            .next()
            .context("Could not parse color")
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

#[async_trait]
impl poise::SlashArgument for Color {
    fn create(builder: &mut serenity::CreateApplicationCommandOption) {
        builder.kind(serenity::CommandOptionType::String);
    }

    async fn extract(
        ctx: &serenity::Context,
        interaction: poise::ApplicationCommandOrAutocompleteInteraction<'_>,
        value: &serenity::json::Value,
    ) -> StdResult<Self, poise::SlashArgError> {
        Ok(Self::from_str(value.as_str().unwrap()).unwrap())
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.into_hex())
    }
}
