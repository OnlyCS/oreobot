use crate::prelude::{serenity::*, *};

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum CacheError {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CacheRequest {
    IsReady,
    GetImpersonation(UserId),
    SetImpersonation(UserId, UserId),
    StopImpersonation(UserId),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum CacheResponse {
    Ready,
    NotReady,
    SetOk,
    Err(String),

    ImpersonationOk(Option<UserId>),
}

#[derive(Debug)]
pub struct CacheServer;

impl ServerMetadata for CacheServer {
    type Request = CacheRequest;
    type Response = CacheResponse;
    type Error = CacheError;

    const HOST: &'static str = "cache";
    const PORT: u16 = 9001;

    const READY_REQUEST: Self::Request = CacheRequest::IsReady;
    const READY_TRUE: Self::Response = CacheResponse::Ready;
    const READY_FALSE: Self::Response = CacheResponse::NotReady;
}
