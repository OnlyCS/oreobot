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

        custom: LoggingRequest::MessageSetImpersonation { source, impersonated, clone } => {
            match database::message::update_impersonation(source, impersonated, clone).await {
                Ok(_) => LoggingResponse::UpdateOk,
                Err(e) => {
                    let err = format!("Failed to update impersionation: {e}");
                    error!("{err}");
                    LoggingResponse::Err(err)
                }
            }
        },

        crud: {
            item: role,
            function_prefix: database,
            response: LoggingResponse::UpdateOk,
            read_response: LoggingResponse::RoleOk(data),
            read_all_response: LoggingResponse::AllRolesOk(data),
        },

        custom: LoggingRequest::RoleSetBlacklisted(role_id) => {
            match database::role::set_blacklisted(role_id).await {
                Ok(_) => LoggingResponse::UpdateOk,
                Err(e) => {
                    let err = format!("Failed to set role blacklisted: {e}");
                    error!("{err}");
                    LoggingResponse::Err(err)
                }
            }
        },

        crud: {
            item: member,
            function_prefix: database,
            response: LoggingResponse::UpdateOk,
            read_response: LoggingResponse::MemberOk(data),
        },

        custom: LoggingRequest::LoggerReady => {
            match database::ready::ready().await {
                Ok(_) => LoggingResponse::Ready,
                Err(e) => {
                    let err = format!("Failed to log ready event: {}", e);
                    error!("{}", err);
                    LoggingResponse::Err(err)
                },
            }
        },

        custom: LoggingRequest::NewsInChatCreate { news, chat } => {
            match database::news_in_chat::create(news, chat).await {
                Ok(_) => LoggingResponse::UpdateOk,
                Err(e) => {
                    let err = format!("Failed to log news in chat: {}", e);
                    error!("{}", err);
                    LoggingResponse::Err(err)
                },
            }
        },

        custom: LoggingRequest::NewsInChatRead(news) => {
            match database::news_in_chat::read(news).await {
                Ok(data) => LoggingResponse::NewsInChatOk(news, data),
                Err(e) => {
                    let err = format!("Failed to get news in chat: {}", e);
                    error!("{}", err);
                    LoggingResponse::Err(err)
                },
            }
        },

        custom: LoggingRequest::NewsInChatReadAll => {
            match database::news_in_chat::all().await {
                Ok(data) => LoggingResponse::AllNewsInChatOk(data),
                Err(e) => {
                    let err = format!("Failed to get news in chat: {}", e);
                    error!("{}", err);
                    LoggingResponse::Err(err)
                },
            }
        },

        custom: LoggingRequest::UserSettingsCreate(user_id, settings) => {
            match database::user_settings::create(user_id, settings).await {
                Ok(()) => LoggingResponse::UpdateOk,
                Err(e) => {
                    let err = format!("Failed to create user settings: {}", e);
                    error!("{}", err);
                    LoggingResponse::Err(err)
                }
            }
        },

        custom: LoggingRequest::UserSettingsUpdate(user_id, settings_update) => {
            match database::user_settings::update(user_id, settings_update).await {
                Ok(()) => LoggingResponse::UpdateOk,
                Err(e) => {
                    let err = format!("Failed to update user settings: {}", e);
                    error!("{}", err);
                    LoggingResponse::Err(err)
                }
            }
        },

        custom: LoggingRequest::UserSettingsRead(user_id) => {
            match database::user_settings::read(user_id).await {
                Ok(data) => LoggingResponse::UserSettingsOk(data),
                Err(e) => {
                    let err = format!("Failed to read user settings: {}", e);
                    error!("{}", err);
                    LoggingResponse::Err(err)
                }
            }
        },

        custom: LoggingRequest::UserSettingsReadAll => {
            match database::user_settings::all().await {
                Ok(data) => LoggingResponse::AllUserSettingsOk(data),
                Err(e) => {
                    let err = format!("Failed to read user settings: {}", e);
                    error!("{}", err);
                    LoggingResponse::Err(err)
                }
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
