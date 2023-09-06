use crate::prelude::*;

#[derive(Error, Debug)]
pub enum PrismaError {
    #[error("prisma query error")]
    PrismaQuery(#[from] QueryError),

    #[error("prisma create error")]
    PrismaCreate(#[from] NewClientError),

    #[error("{0} not found in database")]
    NotFound(String),
}

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("serenity error")]
    Serenity(#[from] serenity::Error),

    #[error("prisma query error")]
    PrismaQuery(#[from] QueryError),

    #[error("prisma create error")]
    PrismaCreate(#[from] NewClientError),

    #[error("prisma relation not fetched error")]
    PrismaRelationNotFetched(#[from] RelationNotFetchedError),

    #[error("settings error")]
    SettingsError(#[from] SettingsError),

    #[error("prisma error")]
    Prisma(#[from] PrismaError),
}

#[derive(Error, Debug)]
pub enum EventError {
    #[error("serde error")]
    Serde(#[from] serde_json::Error),
}

#[derive(Error, Debug)]
pub enum StarboardError {
    #[error("prisma query error")]
    PrismaQuery(#[from] QueryError),

    #[error("prisma create error")]
    PrismaCreate(#[from] NewClientError),

    #[error("prisma relation not fetched error")]
    PrismaRelationNotFetched(#[from] RelationNotFetchedError),

    #[error("the following message with id {0} was not found in the database")]
    MessageNotInDatabase(serenity::MessageId),

    #[error("message clone error")]
    MessageClone(#[from] MessageCloneError),

    #[error("serenity error")]
    Serenity(#[from] serenity::Error),
}

#[derive(Error, Debug)]
pub enum LoggingError {
    #[error("prisma query error")]
    PrismaQuery(#[from] QueryError),

    #[error("prisma create error")]
    PrismaCreate(#[from] NewClientError),

    #[error("prisma relation not fetched error")]
    PrismaRelationNotFetched(#[from] RelationNotFetchedError),

    #[error("serenity error")]
    Serenity(#[from] serenity::Error),

    #[error("channel with id {0} is a thread")]
    ChannelIsThread(serenity::ChannelId),

    #[error("could not find user with id {0} in the database")]
    UserNotInDatabase(serenity::UserId),

    #[error("user with id {0} does not have a color role in the database")]
    UserNoColorRole(serenity::UserId),

    #[error("could not find nci in ctx.cache")]
    NciNotFound,

    #[error("color parse error")]
    Color(#[from] ColorParseError),
}

#[derive(Error, Debug)]
pub enum SettingsError {
    #[error("serde error")]
    Serde(#[from] serde_json::Error),

    #[error("<{0}>::default_value failed")]
    DefaultValueFailed(String),

    #[error("<{0}>::on_change failed")]
    OnChangeFailed(String),
}

#[derive(Error, Debug)]
pub enum ColorParseError {
    #[error("failed to parse from hex {0}")]
    ParseHex(String),

    #[error("failed to parse from color name {0}")]
    ParseName(String),

    #[error("Color::from_str failed to parse {0}")]
    ParseStr(String),
}

#[derive(Error, Debug)]
pub enum MessageCloneError {
    #[error("serenity error")]
    Serenity(#[from] serenity::Error),

    #[error("prisma query error")]
    PrismaQuery(#[from] QueryError),

    #[error("prisma create error")]
    PrismaCreate(#[from] NewClientError),

    #[error("prisma relation not fetched error")]
    PrismaRelationNotFetched(#[from] RelationNotFetchedError),

    #[error("webhook not found while trying to resync")]
    NoWebhook,

    #[error("webhook message not found after cloning")]
    NoWebhookMessage,

    #[error("could not find nci in ctx.cache")]
    NciNotFound,
}

#[derive(Error, Debug)]
pub enum AnyError {
    #[error("serenity error")]
    Serenity(#[from] serenity::Error),

    #[error("prisma query error")]
    PrismaQuery(#[from] QueryError),

    #[error("prisma create error")]
    PrismaCreate(#[from] NewClientError),

    #[error("prisma relation not fetched error")]
    PrismaRelationNotFetched(#[from] RelationNotFetchedError),

    #[error("anyhow error")]
    Anyhow(#[from] anyhow::Error),

    #[error("message clone error")]
    Clone(#[from] MessageCloneError),

    #[error("color parse error")]
    Color(#[from] ColorParseError),

    #[error("command error")]
    Command(#[from] CommandError),

    #[error("event emitter error")]
    Event(#[from] EventError),

    #[error("database logging error")]
    Logging(#[from] LoggingError),

    #[error("settings error")]
    Settings(#[from] SettingsError),

    #[error("starboard error")]
    Starboard(#[from] StarboardError),

    #[error("prisma error")]
    Prisma(#[from] PrismaError),
}

pub trait MakeError<T, E> {
    fn make_error(self, error: E) -> Result<T, E>;
}

impl<T, U, E> MakeError<T, E> for Result<T, U> {
    fn make_error(self, error: E) -> Result<T, E> {
        match self {
            Ok(n) => Ok(n),
            Err(_) => Err(error),
        }
    }
}

impl<T, E> MakeError<T, E> for Option<T> {
    fn make_error(self, error: E) -> Result<T, E> {
        match self {
            Some(n) => Ok(n),
            None => Err(error),
        }
    }
}

macro_rules! bail {
    ($err:expr) => {
        return Err($err);
    };
}

macro_rules! anyhow {
    ($msg:literal $(,)?) => {
        AnyError::Anyhow(anyhow::anyhow!($msg))
    };
    ($err:expr $(,)?) => {
        AnyError::Anyhow(anyhow::anyhow!($err))
	};
	($fmt:expr, $($arg:tt)*) => {
		AnyError::Anyhow(anyhow::anyhow!($fmt, $($arg)*))
	};
}

pub(crate) use anyhow;
pub(crate) use bail;
