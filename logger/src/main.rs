#![feature(error_generic_member_access)]

extern crate oreo_logger;
extern crate oreo_prelude;
extern crate oreo_router;
extern crate tokio;

mod database;
mod error;
mod prelude;

use oreo_logger::{LoggingRequest, LoggingResponse};
use oreo_router::Server;
use prelude::*;

async fn on(request: LoggingRequest) -> LoggingResponse {
    info!("Logging request: {:?}", request);

    match request {
        LoggingRequest::IsReady => LoggingResponse::Ready,
        _ => LoggingResponse::Ok,
    }
}

#[tokio::main]
async fn main() -> Result<(), RouterError> {
    Server::new(on).await?.listen().await?;

    Ok(())
}
