extern crate oreo_prelude;
extern crate oreo_router;
extern crate serde;
extern crate thiserror;
extern crate tokio;

use std::collections::HashMap;

use oreo_cache::{CacheRequest, CacheResponse};
use oreo_prelude::serenity::UserId;
use oreo_router::error::RouterError;
use oreo_router::CacheServer;

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
async fn main() -> Result<(), RouterError> {
    CacheServer::new(Cache::default(), |a, b| {
        Box::pin(async move { on(a, b).await })
    })
    .await?
    .listen()
    .await?;

    Ok(())
}
