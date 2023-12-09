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

    #[error("Error from logging server: {0}")]
    LoggingError(String),

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
}
