#![feature(never_type)]

extern crate oreo_prelude;
extern crate oreo_router;
extern crate serde;
extern crate thiserror;
extern crate tokio;

use std::collections::HashMap;

use oreo_cache::{CacheRequest, CacheResponse};
use oreo_prelude::{serenity::*, *};
use oreo_router::error::RouterError;
use oreo_router::CacheServer;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CacheServerError {
    #[error("Problem with router: {error}")]
    Router {
        #[from]
        error: RouterError,
    },

    #[error("Problem starting logger: {error}")]
    Logger {
        #[from]
        error: SetLoggerError,
    },
}

#[derive(Clone, Debug, Default)]
pub struct Cache {
    pub impersonations: HashMap<UserId, UserId>,
}

async fn on(request: CacheRequest, cache: &mut Cache) -> CacheResponse {
    match request {
        CacheRequest::GetImpersonation(uid) => {
            CacheResponse::ImpersonationOk(cache.impersonations.get(&uid).copied())
        }
        CacheRequest::SetImpersonation(uid, imp) => {
            cache.impersonations.insert(uid, imp);
            CacheResponse::SetOk
        }
        CacheRequest::StopImpersonation(uid) => {
            cache.impersonations.remove(&uid);
            CacheResponse::SetOk
        }
    }
}

#[tokio::main]
async fn main() -> Result<!, CacheServerError> {
    SimpleLogger::new().init()?;

    CacheServer::new(Cache::default(), |a, b| {
        Box::pin(async move { on(a, b).await })
    })
    .await?
    .listen()
    .await?
}
