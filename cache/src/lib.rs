extern crate oreo_prelude;
extern crate oreo_router;
extern crate serde;

use oreo_prelude::serenity::*;
use oreo_router::Request;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CacheRequest {
    GetImpersonation(UserId),
    SetImpersonation(UserId, UserId),
    StopImpersonation(UserId),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CacheResponse {
    Ready,
    NotReady,
    SetOk,
    Err(String),

    ImpersonationOk(Option<UserId>),
}

impl Request for CacheRequest {
    type Response = CacheResponse;

    fn port() -> u16 {
        9001
    }
}
