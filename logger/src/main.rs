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

macro_rules! wire {
    (
        $request_item:ident;
        $($request:pat, $function:expr, $error:expr);*
    ) => {
        match $request_item {
            LoggingRequest::IsReady => LoggingResponse::Ready,
            $(
                $request => {
                    match $function.await {
                        Ok(()) => LoggingResponse::Ok,
                        Err(error) => {
                            let error_string = format!("{}: {}", $error, error);
                            error!("{}", error_string);
                            LoggingResponse::Err(error_string)
                        },
                    }
                }
            )*,
            _ => LoggingResponse::Err("Invalid request".to_string()),
        }
    }
}

async fn on(request: LoggingRequest) -> LoggingResponse {
    info!("Logging request: {:?}", request);

    wire! {
        request;

        LoggingRequest::LogInteractionCreate(interaction), database::interaction::create(interaction), "Failed to log interaction";

        LoggingRequest::LogMessageCreate(message), database::message::create(message), "Failed to log message create";
        LoggingRequest::LogMessageUpdate(message), database::message::update(message), "Failed to log message update";
        LoggingRequest::LogMessageDelete{ message_id, .. }, database::message::delete(message_id), "Failed to log message delete";

        LoggingRequest::LogMemberJoin(member), database::member::join(member), "Failed to log member join";
        LoggingRequest::LogMemberUpdate(member), database::member::update(member), "Failed to log member update";
        LoggingRequest::LogMemberLeave { user, .. }, database::member::leave(user.id), "Failed to log member leave";

        LoggingRequest::LogChannelCreate(channel), database::channel::create(channel), "Failed to log channel create";
        LoggingRequest::LogChannelUpdate(channel), database::channel::update(channel), "Failed to log channel update";
        LoggingRequest::LogChannelDelete(channel), database::channel::delete(channel.id), "Failed to log channel delete";

        LoggingRequest::LogCategoryCreate(category), database::category::create(category), "Failed to log category create";
        LoggingRequest::LogCategoryUpdate(category), database::category::update(category), "Failed to log category update";
        LoggingRequest::LogCategoryDelete(category), database::category::delete(category.id), "Failed to log category delete";

        LoggingRequest::LogReady, database::ready::ready(), "Failed to log ready event"
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
