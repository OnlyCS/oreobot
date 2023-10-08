#![feature(const_option, const_trait_impl)]

#[cfg(feature = "color")]
extern crate color_name;
extern crate itertools;

#[cfg(feature = "log")]
extern crate log;

#[cfg(feature = "prisma")]
extern crate oreo_prisma;

#[cfg(feature = "serenity")]
extern crate serenity as serenity_rs;

#[cfg(feature = "color")]
extern crate serde;

#[cfg(feature = "color")]
extern crate thiserror;

#[cfg(feature = "color-arg")]
extern crate poise;

#[cfg(feature = "color-arg")]
extern crate async_trait;

mod error;

#[cfg(feature = "nci")]
pub mod nci;

#[cfg(feature = "color")]
mod color;

#[cfg(feature = "serenity")]
pub use serenity_rs::all as serenity;

#[cfg(feature = "poise")]
pub use poise::serenity_prelude as serenity;

pub use error::*;

#[cfg(feature = "prisma")]
pub use oreo_prisma::prelude::*;

#[cfg(feature = "prisma")]
pub use oreo_prisma::prisma_client_rust;

#[cfg(feature = "prisma")]
pub use oreo_prisma::prisma_error_convert;

#[cfg(feature = "log")]
pub use log::{debug, error, info, trace, warn};

#[cfg(feature = "color")]
pub use color::{consts as colors, Color, ColorParseError};

pub use itertools::Itertools;
