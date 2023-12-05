#![feature(never_type)]

extern crate oreo_prelude;
extern crate serde;
extern crate serde_json;
extern crate thiserror;
extern crate tokio;

#[cfg(any(feature = "client", feature = "server", feature = "cache-server"))]
mod comms;

pub mod error;

mod prelude {
    pub use crate::{error::*, ServerMetadata};
    pub use oreo_prelude::*;
    pub use serde::{Deserialize, Serialize};
    pub use std::{error::Error, fmt::Debug, marker::PhantomData, sync::Arc};
    pub use thiserror::Error;
    pub use tokio::{
        io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
        net::{TcpListener, TcpStream},
        sync::Mutex,
    };
}

use prelude::*;

#[cfg(feature = "client")]
pub use comms::Client;

#[cfg(feature = "server")]
pub use comms::Server;

#[cfg(feature = "cache-server")]
pub use comms::PersistServer;

pub trait ServerMetadata: Debug + Send + 'static {
    type Request: Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static;
    type Response: Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static + PartialEq;
    type Error: Serialize + for<'de> Deserialize<'de> + Send + Sync + Error + 'static;

    const READY_REQUEST: Self::Request;
    const READY_TRUE: Self::Response;
    const READY_FALSE: Self::Response;

    const HOST: &'static str;
    const PORT: u16;
}
