#![feature(never_type)]

extern crate async_std;
extern crate oreo_prelude;
extern crate serde;
extern crate serde_error;
extern crate serde_json;
extern crate thiserror;
extern crate tokio;

mod error;
mod net;
#[cfg(any(
    feature = "servermeta-logger",
    feature = "servermeta-cache",
    feature = "servermeta-bot"
))]
mod servers;
mod prelude {
    pub use crate::{error::*, ServerMetadata};
    pub use async_std::{
        io::BufReader,
        net::{TcpListener, TcpStream},
    };
    pub use oreo_prelude::*;
    pub use serde::{Deserialize, Serialize};
    pub use std::{collections::HashMap, error::Error, fmt::Debug, marker::PhantomData};
    pub use thiserror::Error;

    #[cfg(feature = "persist-server")]
    pub use tokio::sync::Mutex;

    #[cfg(feature = "persist-server")]
    pub use std::sync::Arc;
}

use prelude::*;

pub trait ServerMetadata: Debug + Send + 'static {
    type Request: Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static + Debug;
    type Response: Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static + PartialEq + Debug;
    type Error: Serialize + for<'de> Deserialize<'de> + Send + Sync + Error + 'static;

    const READY_REQUEST: Self::Request;
    const READY_TRUE: Self::Response;
    const READY_FALSE: Self::Response;

    const HOST: &'static str;
    const PORT: u16;
}

#[cfg(feature = "client")]
pub use net::client::Client;

#[cfg(feature = "server")]
pub use net::server::Server;

#[cfg(feature = "persist-server")]
pub use net::persist_server::PersistServer;

#[cfg(feature = "servermeta-logger")]
pub use servers::logging::*;

#[cfg(feature = "servermeta-cache")]
pub use servers::cache::*;

#[cfg(feature = "servermeta-bot")]
pub use servers::bot::*;

pub use error::*;
