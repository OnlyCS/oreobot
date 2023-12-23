use crate::prelude::*;
use oreo_proc_macros::FromPrismaError;
use std::backtrace::Backtrace;

#[derive(Error, Debug)]
pub enum BotServerError {
    #[error("Problem starting logger: {error}")]
    SetLogger {
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

#[derive(Error, Debug, FromPrismaError)]
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
    StartServer {
        #[from]
        error: RouterError<BotServer>,
        backtrace: Backtrace,
    },

    #[error("Prisma error: {error}")]
    Prisma {
        #[from]
        error: prisma::Error,
        backtrace: Backtrace,
    },

    #[error("Event error: {error}")]
    Event {
        #[from]
        error: EventError,
        backtrace: Backtrace,
    },

    #[error("Illegal argument: {0}")]
    IllegalArgument(String),

    #[error("Error communicating with cache server: {error}")]
    CacheServer {
        #[from]
        error: RouterError<CacheServer>,
        backtrace: Backtrace,
    },

    #[error("Error communicating with logging server: {error}")]
    LoggerServer {
        #[from]
        error: RouterError<LoggingServer>,
        backtrace: Backtrace,
    },

    #[error("Admin permissions are necessary to run this command")]
    AdminRequired,

    #[error("No color role for user: {0}")]
    NoColorRole(String),

    #[error("Error parsing color: {error}")]
    ColorParse {
        #[from]
        error: ColorParseError,
        backtrace: Backtrace,
    },

    #[error("This command can only be used in a guild")]
    NotInGuild,

    #[error("Role ({{ id: {0} }}) could not be found")]
    RoleNotFound(RoleId),
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
    LoggerServer {
        #[from]
        error: RouterError<LoggingServer>,
        backtrace: Backtrace,
    },

    #[error("Cannot clone messages containing components")]
    NoComponents,
}

#[derive(Error, Debug, FromPrismaError)]
pub enum EventError {
    #[error("Serenity error: {error}")]
    Serenity {
        #[from]
        error: serenity::Error,
        backtrace: Backtrace,
    },

    #[error("Error communicating with logging server: {error}")]
    LoggingServer {
        #[from]
        error: RouterError<LoggingServer>,
        backtrace: Backtrace,
    },

    #[error("Error communicating with cache server: {error}")]
    CacheServer {
        #[from]
        error: RouterError<CacheServer>,
        backtrace: Backtrace,
    },

    #[error("Prisma error: {error}")]
    Prisma {
        #[from]
        error: prisma::Error,
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

    #[error("Error with starboard: {error}")]
    Starboard {
        #[from]
        error: StarboardError,
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

#[derive(Error, Debug)]
pub enum StarboardError {
    #[error("Error with logging server: {error}")]
    LoggingServer {
        #[from]
        error: RouterError<LoggingServer>,
        backtrace: Backtrace,
    },

    #[error("Error cloning message: {error}")]
    MessageClone {
        #[from]
        error: MessageCloneError,
        backtrace: Backtrace,
    },

    #[error("Cannot star a message twice")]
    DoubleStar,
}
