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
        $request_item:ident,
        $({
            request: $request:pat,
            function: $function:expr,
            result_ok: $ok_arg:pat,
            response: $response:expr,
            error: $error:expr
        }),*
    ) => {
        match $request_item {
            LoggingRequest::IsReady => LoggingResponse::Ready,
            $(
                $request => {
                    match $function.await {
                        Ok($ok_arg) => $response,
                        Err(error) => {
                            let error_string = format!("{}: {}", $error, error);
                            error!("{}", error_string);
                            LoggingResponse::Err(error_string)
                        },
                    }
                }
            )*,
        }
    }
}

async fn on(request: LoggingRequest) -> LoggingResponse {
    info!("Logging request: {:?}", request);

    wire! {
        request,

        {
            request: LoggingRequest::LogInteractionCreate(interaction),
            function: database::interaction::create(interaction),
            result_ok: (),
            response: LoggingResponse::UpdateOk,
            error: "Failed to log interaction"
        },

        {
            request: LoggingRequest::LogMessageCreate(message),
            function: database::message::create(message),
            result_ok: (),
            response: LoggingResponse::UpdateOk,
            error: "Failed to log message create"
        },

        {
            request: LoggingRequest::LogMessageUpdate(message),
            function: database::message::update(message),
            result_ok: (),
            response: LoggingResponse::UpdateOk,
            error: "Failed to log message update"
        },

        {
            request: LoggingRequest::LogMessageDelete { message_id, .. },
            function: database::message::delete(message_id),
            result_ok: (),
            response: LoggingResponse::UpdateOk,
            error: "Failed to log message delete"
        },

        {
            request: LoggingRequest::LogMemberJoin(member),
            function: database::member::join(member),
            result_ok: (),
            response: LoggingResponse::UpdateOk,
            error: "Failed to log member join"
        },

        {
            request: LoggingRequest::LogMemberUpdate(member),
            function: database::member::update(member),
            result_ok: (),
            response: LoggingResponse::UpdateOk,
            error: "Failed to log member update"
        },

        {
            request: LoggingRequest::LogMemberLeave { user, .. },
            function: database::member::leave(user.id),
            result_ok: (),
            response: LoggingResponse::UpdateOk,
            error: "Failed to log member leave"
        },

        {
            request: LoggingRequest::LogCategoryCreate(channel),
            function: database::category::create(channel),
            result_ok: (),
            response: LoggingResponse::UpdateOk,
            error: "Failed to log category create"
        },

        {
            request: LoggingRequest::LogCategoryUpdate(channel),
            function: database::category::update(channel),
            result_ok: (),
            response: LoggingResponse::UpdateOk,
            error: "Failed to log category update"
        },

        {
            request: LoggingRequest::LogCategoryDelete(channel),
            function: database::category::delete(channel.id),
            result_ok: (),
            response: LoggingResponse::UpdateOk,
            error: "Failed to log category delete"
        },

        {
            request: LoggingRequest::LogChannelCreate(channel),
            function: database::channel::create(channel),
            result_ok: (),
            response: LoggingResponse::UpdateOk,
            error: "Failed to log channel create"
        },

        {
            request: LoggingRequest::LogChannelUpdate(channel),
            function: database::channel::update(channel),
            result_ok: (),
            response: LoggingResponse::UpdateOk,
            error: "Failed to log channel update"
        },

        {
            request: LoggingRequest::LogChannelDelete(channel),
            function: database::channel::delete(channel.id),
            result_ok: (),
            response: LoggingResponse::UpdateOk,
            error: "Failed to log channel delete"
        },

        {
            request: LoggingRequest::LogRoleCreate(role),
            function: database::role::create(role),
            result_ok: (),
            response: LoggingResponse::UpdateOk,
            error: "Failed to log role create"
        },

        {
            request: LoggingRequest::LogRoleUpdate(role),
            function: database::role::update(role),
            result_ok: (),
            response: LoggingResponse::UpdateOk,
            error: "Failed to log role update"
        },

        {
            request: LoggingRequest::LogRoleDelete { role_id, .. },
            function: database::role::delete(role_id),
            result_ok: (),
            response: LoggingResponse::UpdateOk,
            error: "Failed to log role delete"
        },

        {
            request: LoggingRequest::LogReady,
            function: database::ready::ready(),
            result_ok: (),
            response: LoggingResponse::UpdateOk,
            error: "Failed to log ready"
        },

        // get params

        {
            request: LoggingRequest::GetInteraction(interaction_id),
            function: database::interaction::get(interaction_id),
            result_ok: interaction,
            response: LoggingResponse::InteractionOk(interaction),
            error: "Failed to get interaction"
        },

        {
            request: LoggingRequest::GetCategory(channel_id),
            function: database::category::get(channel_id),
            result_ok: category,
            response: LoggingResponse::CategoryOk(category),
            error: "Failed to get category"
        },

        {
            request: LoggingRequest::GetChannel(channel_id),
            function: database::channel::get(channel_id),
            result_ok: channel,
            response: LoggingResponse::ChannelOk(channel),
            error: "Failed to get channel"
        },

        {
            request: LoggingRequest::GetMessage(message_id),
            function: database::message::get(message_id),
            result_ok: message,
            response: LoggingResponse::MessageOk(message),
            error: "Failed to get message"
        },

        {
            request: LoggingRequest::GetRole(role_id),
            function: database::role::get(role_id),
            result_ok: role,
            response: LoggingResponse::RoleOk(role),
            error: "Failed to get role"
        },

        {
            request: LoggingRequest::GetMember(user_id),
            function: database::member::get(user_id),
            result_ok: member,
            response: LoggingResponse::MemberOk(member),
            error: "Failed to get member"
        }
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
