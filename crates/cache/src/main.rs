#![feature(never_type, error_generic_member_access)]

extern crate oreo_prelude;
extern crate oreo_router;
extern crate serde;
extern crate thiserror;
extern crate tokio;

use std::backtrace::Backtrace;
use std::collections::HashMap;

use oreo_cache::{CacheError, CacheRequest, CacheResponse, CacheServer};
use oreo_prelude::{serenity::*, *};
use oreo_router::error::RouterError;
use oreo_router::PersistServer;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CacheBinError {
    #[error("Error starting simple_logger: {error}")]
    SimpleLoggerError {
        #[from]
        error: SetLoggerError,
        backtrace: Backtrace,
    },

    #[error("Error with router: {error}")]
    LoggerError {
        #[from]
        error: RouterError<CacheServer>,
        backtrace: Backtrace,
    },
}

#[derive(Clone, Debug, Default)]
pub struct Cache {
    pub impersonations: HashMap<UserId, UserId>,
}

async fn on(request: CacheRequest, cache: &mut Cache) -> Result<CacheResponse, CacheError> {
    Ok(match request {
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
        CacheRequest::IsReady => CacheResponse::Ready,
    })
}

#[tokio::main]
async fn main() -> Result<!, CacheBinError> {
    SimpleLogger::new().init()?;

    PersistServer::new(Cache::default(), |a, b| {
        Box::pin(async move { on(a, b).await })
    })
    .await?
    .listen()
    .await?
}
