use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::Debug;

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
