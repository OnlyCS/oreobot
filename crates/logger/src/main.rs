#![feature(error_generic_member_access, trace_macros, never_type, let_chains)]

extern crate oreo_prelude;
extern crate oreo_router;
extern crate tokio;

mod database;
mod error;
mod prelude;

use database::{
    category, channel, interaction, member, message, message_clone, ready, role, user_settings,
};

use oreo_proc_macros::wire;
use prelude::*;

async fn on(request: LoggingRequest) -> Result<LoggingResponse, LoggerError> {
    let mut bot = Client::<BotServer>::new().await?;

    wire! {
        request,

        IsReady => _ => LoggingResponse::Ready,

        // interaction
        InteractionCreate(i) => interaction::create(i) => LoggingResponse::UpdateOk,
        InteractionRead(i) => interaction::read(i) => LoggingResponse::InteractionOk(out),

        // category
        CategoryCreate(c) => category::create(c) => LoggingResponse::UpdateOk,
        CategoryRead(c) => category::read(c) => LoggingResponse::CategoryOk(out),
        CategoryUpdate(c) => category::update(c) => LoggingResponse::UpdateOk,
        CategoryDelete(c) => category::delete(c) => LoggingResponse::UpdateOk,

        // channel
        ChannelCreate(c) => channel::create(c) => LoggingResponse::UpdateOk,
        ChannelRead(c) => channel::read(c) => LoggingResponse::ChannelOk(out),
        ChannelUpdate(c) => channel::update(c) => LoggingResponse::UpdateOk,
        ChannelDelete(c) => channel::delete(c) => LoggingResponse::UpdateOk,

        // message
        MessageCreate(m) => message::create(m) => LoggingResponse::UpdateOk,
        MessageRead(m) => message::read(m) => LoggingResponse::MessageOk(out),
        MessageUpdate(m) => message::update(m) => LoggingResponse::UpdateOk,
        MessageDelete(m) => message::delete(m) => LoggingResponse::UpdateOk,

        // role
        RoleCreate(r) => role::create(r) => LoggingResponse::UpdateOk,
        RoleSetBlacklisted(r) => role::set_blacklisted(r) => LoggingResponse::UpdateOk,
        RoleRead(r) => role::read(r) => LoggingResponse::RoleOk(out),
        RoleReadAll => role::all() => LoggingResponse::AllRolesOk(out),
        RoleUpdate(r) => role::update(r) => LoggingResponse::UpdateOk,
        RoleDelete(r) => role::delete(r, &mut bot) => LoggingResponse::UpdateOk,

        // member
        MemberCreate(m) => member::create(m, &mut bot) => LoggingResponse::UpdateOk,
        MemberRead(m) => member::read(m) => LoggingResponse::MemberOk(out),
        MemberUpdate(m) => member::update(m, &mut bot) => LoggingResponse::UpdateOk,
        MemberDelete(m) => member::delete(m, &mut bot) => LoggingResponse::UpdateOk,

        // user settings
        UserSettingsCreate(u, s) => user_settings::create(u, s) => LoggingResponse::UpdateOk,
        UserSettingsRead(u) => user_settings::read(u) => LoggingResponse::UserSettingsOk(out),
        UserSettingsReadAll => user_settings::all() => LoggingResponse::AllUserSettingsOk(out),
        UserSettingsUpdate(u, s) => user_settings::update(u, s) => LoggingResponse::UpdateOk,

        // message clone
        MessageCloneCreate { source, clone, destination, reason, update, update_delete }
            => message_clone::create(source, clone, destination, reason, update, update_delete)
            => LoggingResponse::UpdateOk,
        MessageCloneRead { clone } => message_clone::read(clone) => LoggingResponse::MessageCloneOk(out),
        MessageCloneReadAll => message_clone::all() => LoggingResponse::AllMessageClonesOk(out),

        ReadyEvent => ready::ready(&mut bot) => LoggingResponse::UpdateOk
    }
}

async fn on_map(request: LoggingRequest) -> Result<LoggingResponse, serde_error::Error> {
    let result = on(request).await;
    result.map_err(|error| serde_error::Error::new(&error))
}

#[tokio::main]
async fn main() -> Result<!, LoggerServerError> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Warn)
        .with_threads(true)
        .with_module_level("oreo_logger", log::LevelFilter::Debug)
        .with_module_level("oreo_router", log::LevelFilter::Debug)
        .init()?;

    Server::new(|request| async move { on_map(request).await })
        .await?
        .listen()
        .await?
}
