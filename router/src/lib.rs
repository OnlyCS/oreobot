extern crate oreo_prelude;
extern crate serde;
extern crate serde_json;
extern crate thiserror;
extern crate tokio;

#[cfg(all(feature = "server", feature = "cache-server"))]
compile_error!("Cannot enable both server and cache-server features");

mod comms;
pub mod error;
mod request;

#[cfg(feature = "client")]
pub use comms::Client;

#[cfg(any(feature = "server", feature = "cache-server"))]
pub use comms::Server;

pub use request::Request;
