use oreo_prelude::prisma_error::PrismaError;

use crate::prelude::*;
use std::backtrace::Backtrace;

#[derive(Error, Debug)]
pub enum BotServerError {
    #[error("Problem starting logger: {error}")]
    Logger {
        #[from]
        error: SetLoggerError,
        backtrace: Backtrace,
    },

    #[error("Serenity error: {error}")]
    Serenity {
        #[from]
        error: serenity::Error,
        backtrace: Backtrace,
    },
}

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("Serenity error: {error}")]
    Serenity {
        #[from]
        error: serenity::Error,
        backtrace: Backtrace,
    },

    #[error("Error sending event: {error}")]
    MpmcSend {
        #[from]
        error: async_channel::SendError<mpmc::MpmcData>,
        backtrace: Backtrace,
    },

    #[error("Error starting server: {error}")]
    Server {
        #[from]
        error: RouterError<BotServer>,
        backtrace: Backtrace,
    },

    #[error("Event error: {error}")]
    EventError {
        #[from]
        error: EventError,
        backtrace: Backtrace,
    },
}

#[derive(Error, Debug)]
pub enum CloneError {
    #[error("Serenity error: {error}")]
    Serenity {
        #[from]
        error: serenity::Error,
        backtrace: Backtrace,
    },

    #[error("Logger error: {error}")]
    Router {
        #[from]
        error: RouterError<LoggingServer>,
        backtrace: Backtrace,
    },

    #[error("Cannot clone messages containing components")]
    NoComponents,
}

#[derive(Error, Debug)]
pub enum EventError {
    #[error("Serenity error: {error}")]
    Serenity {
        #[from]
        error: serenity::Error,
        backtrace: Backtrace,
    },

    #[error("Error communicating with logging server: {error}")]
    LoggingServerError {
        #[from]
        error: RouterError<LoggingServer>,
        backtrace: Backtrace,
    },

    #[error("Prisma error: {error}")]
    Prisma {
        #[from]
        error: PrismaError,
        backtrace: Backtrace,
    },

    #[error("Unwanted Event")]
    UnwantedEvent,
}

prisma_error_convert!(EventError);
