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

    #[error("Illegal argument: {0}")]
    IllegalArgument(String),

    #[error("Error communicating with cache server: {error}")]
    CacheServerError {
        #[from]
        error: RouterError<CacheServer>,
        backtrace: Backtrace,
    },
}

#[derive(Error, Debug)]
pub enum MessageCloneError {
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

    #[error("Error communicating with cache server: {error}")]
    CacheServerError {
        #[from]
        error: RouterError<CacheServer>,
        backtrace: Backtrace,
    },

    #[error("Prisma error: {error}")]
    Prisma {
        #[from]
        error: PrismaError,
        backtrace: Backtrace,
    },

    #[error("Error cloning message: {error}")]
    MessageClone {
        #[from]
        error: MessageCloneError,
        backtrace: Backtrace,
    },

    #[error("Error cloning news message: {error}")]
    NewsClone {
        #[from]
        error: NewsCloneError,
        backtrace: Backtrace,
    },

    #[error("Unwanted Event")]
    UnwantedEvent,
}

#[derive(Error, Debug)]
pub enum NewsCloneError {
    #[error("Provided message not in news channel")]
    IncorrectChannel,

    #[error("Error cloning message: {error}")]
    MessageClone {
        #[from]
        error: MessageCloneError,
        backtrace: Backtrace,
    },

    #[error("Serenity error: {error}")]
    Serenity {
        #[from]
        error: serenity::Error,
        backtrace: Backtrace,
    },
}

prisma_error_convert!(EventError);
