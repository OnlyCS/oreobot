use std::backtrace::Backtrace;

use crate::prelude::*;

pub use oreo_router::error::RouterError;

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
}

#[derive(Error, Debug)]
pub enum MessageCloneError {
    #[error("Serenity error: {error}")]
    Serenity {
        #[from]
        error: serenity::Error,
        backtrace: Backtrace,
    },

    #[error("Router error: {error}")]
    Router {
        #[from]
        error: RouterError,
        backtrace: Backtrace,
    },

    #[error("Error from logging server: {0}")]
    LoggingError(String),

    #[error("Cannot clone messages containing components")]
    NoComponents,
}
