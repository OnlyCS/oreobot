#![feature(error_generic_member_access, trace_macros)]

extern crate oreo_logger;
extern crate oreo_prelude;
extern crate oreo_router;
extern crate tokio;

mod database;
mod error;
mod prelude;

use oreo_logger::{LoggingRequest, LoggingResponse};
use oreo_proc_macros::wire;
use oreo_router::Server;
use prelude::*;

async fn on(request: LoggingRequest) -> LoggingResponse {
    info!("Logging request: {:?}", request);

    wire! {
        request,
        LoggingRequest,

        custom: LoggingRequest::IsReady => LoggingResponse::Ready,

        cr: {
            item: interaction,
            function_prefix: database,
            response: LoggingResponse::UpdateOk,
            read_response: LoggingResponse::InteractionOk(data),
        },

        crud: {
            item: category,
            function_prefix: database,
            response: LoggingResponse::UpdateOk,
            read_response: LoggingResponse::CategoryOk(data),
        },

        crud: {
            item: channel,
            function_prefix: database,
            response: LoggingResponse::UpdateOk,
            read_response: LoggingResponse::ChannelOk(data),
        },

        crud: {
            item: message,
            function_prefix: database,
            response: LoggingResponse::UpdateOk,
            read_response: LoggingResponse::MessageOk(data),
        },

        crud: {
            item: role,
            function_prefix: database,
            response: LoggingResponse::UpdateOk,
            read_response: LoggingResponse::RoleOk(data),
        },

        crud: {
            item: member,
            function_prefix: database,
            response: LoggingResponse::UpdateOk,
            read_response: LoggingResponse::MemberOk(data),
        },

        custom: LoggingRequest::LogReady => {
            match database::ready::ready().await {
                Ok(_) => LoggingResponse::UpdateOk,
                Err(e) => LoggingResponse::Err(format!("Failed to log ready event: {}", e)),
            }
        },
    }
}

#[tokio::main]
async fn main() -> Result<(), RouterError> {
    Server::new(|request| async move { on(request).await })
        .await?
        .listen()
        .await?;

    Ok(())
}
