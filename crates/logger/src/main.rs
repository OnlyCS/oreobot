#![feature(error_generic_member_access, trace_macros, never_type, let_chains)]

extern crate oreo_prelude;
extern crate oreo_router;
extern crate tokio;

mod database;
mod error;
mod prelude;

use oreo_proc_macros::wire;
use oreo_router::Server;
use prelude::*;

async fn on(request: LoggingRequest) -> Result<LoggingResponse, LoggerError> {
    info!("Logging request: {:?}", request);

    wire! {
        request,
        LoggingRequest,

        custom: LoggingRequest::IsReady => Ok(LoggingResponse::Ready),

        cr: {
            item: interaction,
            response: LoggingResponse::UpdateOk,
            read_response: LoggingResponse::InteractionOk,
        },

        crud: {
            item: category,
            response: LoggingResponse::UpdateOk,
            read_response: LoggingResponse::CategoryOk,
        },

        crud: {
            item: channel,
            response: LoggingResponse::UpdateOk,
            read_response: LoggingResponse::ChannelOk,
        },

        crud: {
            item: message,
            response: LoggingResponse::UpdateOk,
            read_response: LoggingResponse::MessageOk,
        },

        crud: {
            item: role,
            response: LoggingResponse::UpdateOk,
            read_response: LoggingResponse::RoleOk,
            read_all_response: LoggingResponse::AllRolesOk,
        },

        custom: LoggingRequest::RoleSetBlacklisted(role_id) => {
            database::role::set_blacklisted(role_id).await?;
            Ok(LoggingResponse::UpdateOk)
        },

        crud: {
            item: member,
            response: LoggingResponse::UpdateOk,
            read_response: LoggingResponse::MemberOk,
        },

        custom: LoggingRequest::LoggerReady => {
            database::ready::ready().await?;
            Ok(LoggingResponse::UpdateOk)
        },

        custom: LoggingRequest::UserSettingsCreate(user_id, settings) => {
            database::user_settings::create(user_id, settings).await?;
            Ok(LoggingResponse::UpdateOk)
        },

        custom: LoggingRequest::UserSettingsUpdate(user_id, settings_update) => {
            database::user_settings::update(user_id, settings_update).await?;
            Ok(LoggingResponse::UpdateOk)
        },

        custom: LoggingRequest::UserSettingsRead(user_id) => {
            let data = database::user_settings::read(user_id).await?;
            Ok(LoggingResponse::UserSettingsOk(data))
        },

        custom: LoggingRequest::UserSettingsReadAll => {
            let data = database::user_settings::all().await?;
            Ok(LoggingResponse::AllUserSettingsOk(data))
        },

        custom: LoggingRequest::MessageCloneCreate { source, clone, destination, reason, update, update_delete } => {
            database::message_clone::create(source, clone, destination, reason, update, update_delete).await?;
            Ok(LoggingResponse::UpdateOk)
        },
    }
}

async fn on_map(request: LoggingRequest) -> Result<LoggingResponse, serde_error::Error> {
    let result = on(request).await;
    result.map_err(|error| serde_error::Error::new(&error))
}

#[tokio::main]
async fn main() -> Result<!, LoggerServerError> {
    SimpleLogger::new().init()?;

    Server::new(|request| async move { on_map(request).await })
        .await?
        .listen()
        .await?
}
