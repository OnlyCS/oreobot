#![feature(never_type)]

extern crate oreo_prelude;
extern crate serde;
extern crate serde_json;
extern crate thiserror;
extern crate tokio;

mod comms;
pub mod error;
mod server;

#[cfg(feature = "client")]
pub use comms::Client;

#[cfg(feature = "server")]
pub use comms::Server;

#[cfg(feature = "cache-server")]
pub use comms::PersistServer;

pub use server::ServerMetadata;
