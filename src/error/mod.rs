use crate::prelude::*;

#[derive(Error, Debug)]
pub enum PrismaError {
    #[error("prisma query error: {0}")]
    PrismaQuery(#[from] QueryError),

    #[error("prisma create error: {0}")]
    PrismaCreate(#[from] NewClientError),

    #[error("{0} not found in database")]
    NotFound(String),
}

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("serenity error, {0}")]
    Serenity(#[from] serenity::Error),

    #[error("prisma query error, {0}")]
    PrismaQuery(#[from] QueryError),

    #[error("prisma create error, {0}")]
    PrismaCreate(#[from] NewClientError),

    #[error("prisma relation not fetched error, {0}")]
    PrismaRelationNotFetched(#[from] RelationNotFetchedError),

    #[error("settings error, {0}")]
    SettingsError(#[from] CacheError),

    #[error("prisma error, {0}")]
    Prisma(#[from] PrismaError),

    #[error("error while running command: {description}")]
    RuntimeError {
        title: &'static str,
        description: &'static str,
    },

    #[error("warning while running command: {description}")]
    RuntimeWarning {
        title: &'static str,
        description: &'static str,
    },

    #[error("starboard error, {0}")]
    Starboard(#[from] StarboardError),
}

#[derive(Error, Debug)]
pub enum EventError {
    #[error("serde error {0}")]
    Serde(#[from] serde_json::Error),
}

#[derive(Error, Debug)]
pub enum StarboardError {
    #[error("prisma query error {0}")]
    PrismaQuery(#[from] QueryError),

    #[error("prisma create error: {0}")]
    PrismaCreate(#[from] NewClientError),

    #[error("prisma relation not fetched error")]
    PrismaRelationNotFetched(#[from] RelationNotFetchedError),

    #[error("the following message with id {0} was not found in the database")]
    MessageNotInDatabase(serenity::MessageId),

    #[error("message clone error")]
    MessageClone(#[from] MessageCloneError),

    #[error("serenity error {0}")]
    Serenity(#[from] serenity::Error),

    #[error("clone builder unfinished")]
    UnfinishedBuilder(#[from] UnfinishedBuilderError),
}

#[derive(Error, Debug)]
pub enum LoggingError {
    #[error("prisma query error: {0}")]
    PrismaQuery(#[from] QueryError),

    #[error("prisma create error: {0}")]
    PrismaCreate(#[from] NewClientError),

    #[error("prisma relation not fetched error")]
    PrismaRelationNotFetched(#[from] RelationNotFetchedError),

    #[error("serenity error {0}")]
    Serenity(#[from] serenity::Error),

    #[error("channel with id {0} is a thread")]
    ChannelIsThread(serenity::ChannelId),

    #[error("could not find user with id {0} in the database")]
    UserNotInDatabase(serenity::UserId),

    #[error("user with id {0} does not have a color role in the database")]
    UserNoColorRole(serenity::UserId),

    #[error("could not find nci in ctx.cache")]
    NciNotFound,

    #[error("color parse error: {0}")]
    Color(#[from] ColorParseError),

    #[error("cache error: {0}")]
    Cache(#[from] CacheError),

    #[error("{0} not found in database")]
    NotFound(String),

    #[error("warning: user with id {0} impersonated, skipping")]
    UserImpersonated(serenity::UserId),
}

#[derive(Error, Debug)]
pub enum CacheError {
    #[error("serde error: {0}")]
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
    #[error("serenity error: {0}")]
    Serenity(#[from] serenity::Error),

    #[error("prisma query error: {0}")]
    PrismaQuery(#[from] QueryError),

    #[error("prisma create error: {0}")]
    PrismaCreate(#[from] NewClientError),

    #[error("prisma relation not fetched error: {0}")]
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
    #[error("serenity error {0}")]
    Serenity(#[from] serenity::Error),

    #[error("prisma query error: {0}")]
    PrismaQuery(#[from] QueryError),

    #[error("prisma create error: {0}")]
    PrismaCreate(#[from] NewClientError),

    #[error("prisma relation not fetched error")]
    PrismaRelationNotFetched(#[from] RelationNotFetchedError),

    #[error("anyhow error")]
    Anyhow(#[from] anyhow::Error),

    #[error("message clone error")]
    Clone(#[from] MessageCloneError),

    #[error("color parse error: {0}")]
    Color(#[from] ColorParseError),

    #[error("command error: {0}")]
    Command(#[from] CommandError),

    #[error("event emitter error: {0}")]
    Event(#[from] EventError),

    #[error("database logging error: {0}")]
    Logging(#[from] LoggingError),

    #[error("settings error: {0}")]
    Settings(#[from] CacheError),

    #[error("starboard error: {0}")]
    Starboard(#[from] StarboardError),

    #[error("prisma error: {0}")]
    Prisma(#[from] PrismaError),

    #[error("clone builder unfinished: {0}")]
    UnfinishedBuilder(#[from] UnfinishedBuilderError),
}

#[derive(Error, Debug)]
#[error("required field(s) {0:?} not found in database")]
pub struct UnfinishedBuilderError(pub Vec<&'static str>);

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
