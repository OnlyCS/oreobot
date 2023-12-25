#[cfg(feature = "color")]
extern crate color_name;
extern crate itertools;

#[cfg(feature = "log")]
pub extern crate log;

#[cfg(feature = "log")]
extern crate simple_logger;

#[cfg(feature = "prisma")]
extern crate oreo_prisma;

#[cfg(feature = "serenity")]
extern crate serenity as serenity_rs;

#[cfg(any(feature = "color", feature = "user-settings"))]
extern crate serde;

#[cfg(feature = "color")]
extern crate thiserror;

#[cfg(feature = "color-arg")]
extern crate poise;

#[cfg(feature = "color-arg")]
extern crate async_trait;

#[cfg(feature = "futures")]
extern crate futures;

mod debug;
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
pub use oreo_prisma::{prelude::*, prisma_client_rust};

#[cfg(feature = "log")]
pub use log::{debug, error, info, trace, warn, SetLoggerError};

#[cfg(feature = "log")]
pub use simple_logger::SimpleLogger;

#[cfg(feature = "color")]
pub use color::{consts as colors, Color, ColorParseError};

#[cfg(feature = "futures")]
pub use futures::{future::BoxFuture, prelude::*};

pub use debug::*;
pub use itertools::Itertools;
pub use std::collections::{HashMap, HashSet};
