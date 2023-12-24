use crate::prelude::*;
use oreo_proc_macros::FromPrismaError;
use std::backtrace::Backtrace;

#[derive(Error, Debug, FromPrismaError)]
pub enum MessageLogError {
    #[error("Problem with router: {error}")]
    Router {
        #[from]
        error: RouterError<LoggingServer>,
        backtrace: Backtrace,
    },

    #[error("Warning: Message ({{ id: {0} }}) is owned by a webhook, skipping")]
    WebhookMessage(serenity::MessageId),

    #[error("Message ({{ id: {0} }}) not found")]
    NotFound(serenity::MessageId),

    #[error("Warning: Skipping delete for message ({{ id: {0} }}), impersonated")]
    MessageImpersonated(serenity::MessageId),

    #[error("Problem with database: {error}")]
    Database {
        #[from]
        error: prisma::Error,
        backtrace: Backtrace,
    },
}

#[derive(Error, Debug, FromPrismaError)]
pub enum CategoryLogError {
    #[error("Problem with database: {error}")]
    Database {
        #[from]
        error: prisma::Error,
        backtrace: Backtrace,
    },

    #[error("Problem communicating with bot: {error}")]
    Bot {
        #[from]
        error: RouterError<BotServer>,
        backtrace: Backtrace,
    },

    #[error("Category ({{ id: {0} }}) not found")]
    NotFound(serenity::ChannelId),
}

#[derive(Error, Debug, FromPrismaError)]
pub enum ChannelLogError {
    #[error("Warning: Channel ({{ id: {0} }}) is a thread, skipping")]
    Thread(serenity::ChannelId),

    #[error("Problem with database: {error}")]
    Database {
        #[from]
        error: prisma::Error,
        backtrace: Backtrace,
    },

    #[error("Problem communicating with bot: {error}")]
    Bot {
        #[from]
        error: RouterError<BotServer>,
        backtrace: Backtrace,
    },

    #[error("Channel ({{ id: {0} }}) not found")]
    NotFound(serenity::ChannelId),
}

#[derive(Error, Debug, FromPrismaError)]
pub enum InteractionLogError {
    #[error("Problem with database: {error}")]
    Database {
        #[from]
        error: prisma::Error,
        backtrace: Backtrace,
    },

    #[error("Interaction ({{ id: {0} }}) not found")]
    NotFound(serenity::InteractionId),
}

#[derive(Error, Debug, FromPrismaError)]
pub enum MemberLogError {
    #[error("Problem with database: {error}")]
    Database {
        #[from]
        error: prisma::Error,
        backtrace: Backtrace,
    },

    #[error("Problem communicating with bot: {error}")]
    Bot {
        #[from]
        error: RouterError<BotServer>,
        backtrace: Backtrace,
    },

    #[error("Member ({{ id: {0} }}) not found")]
    NotFound(serenity::UserId),

    #[error("Member ({{ id: {0} }}) has no color role")]
    NoColorRole(serenity::UserId),

    #[error("Failed to update user settings: {error}")]
    UpdateUserSettings {
        #[from]
        error: UserSettingsLogError,
        backtrace: Backtrace,
    },
}

#[derive(Error, Debug, FromPrismaError)]
pub enum RoleLogError {
    #[error("Problem with database: {error}")]
    Database {
        #[from]
        error: prisma::Error,
        backtrace: Backtrace,
    },

    #[error("Problem communicating with bot: {error}")]
    Bot {
        #[from]
        error: RouterError<BotServer>,
        backtrace: Backtrace,
    },

    #[error("Role ({{ id: {0} }}) not found")]
    RoleNotFound(serenity::RoleId),

    #[error("Error parsing role color: {error}")]
    ColorParse {
        #[from]
        error: ColorParseError,
        backtrace: Backtrace,
    },
}

#[derive(Error, Debug)]
pub enum ReadyEventError {
    #[error("Problem updating roles: {error}")]
    RoleUpdate {
        #[from]
        error: RoleLogError,
        backtrace: Backtrace,
    },

    #[error("Problem updating members: {error}")]
    MemberUpdate {
        #[from]
        error: MemberLogError,
        backtrace: Backtrace,
    },

    #[error("Problem updating channels: {error}")]
    ChannelUpdate {
        #[from]
        error: ChannelLogError,
        backtrace: Backtrace,
    },

    #[error("Problem updating categories: {error}")]
    CategoryUpdate {
        #[from]
        error: CategoryLogError,
        backtrace: Backtrace,
    },
}

#[derive(Error, Debug, FromPrismaError)]
pub enum UserSettingsLogError {
    #[error("Problem with database: {error}")]
    Database {
        #[from]
        error: prisma::Error,
        backtrace: Backtrace,
    },
}

#[derive(Error, Debug, FromPrismaError)]
pub enum MessageCloneLogError {
    #[error("Problem with database: {error}")]
    Database {
        #[from]
        error: prisma::Error,
        backtrace: Backtrace,
    },

    #[error("No message clone found with clone id {0}")]
    NotFound(serenity::MessageId),
}

#[derive(Error, Debug)]
pub enum LoggerServerError {
    #[error("Problem with router: {error}")]
    Router {
        #[from]
        error: RouterError<LoggingServer>,
        backtrace: Backtrace,
    },

    #[error("Problem starting logger: {error}")]
    Logger {
        #[from]
        error: SetLoggerError,
        backtrace: Backtrace,
    },
}

#[derive(Error, Debug)]
pub enum LoggerError {
    #[error("Error logging message: {error}")]
    MessageLogError {
        #[from]
        error: MessageLogError,
        backtrace: Backtrace,
    },

    #[error("Error logging category: {error}")]
    CategoryLogError {
        #[from]
        error: CategoryLogError,
        backtrace: Backtrace,
    },

    #[error("Error logging channel: {error}")]
    ChannelLogError {
        #[from]
        error: ChannelLogError,
        backtrace: Backtrace,
    },

    #[error("Error logging interaction: {error}")]
    InteractionLogError {
        #[from]
        error: InteractionLogError,
        backtrace: Backtrace,
    },

    #[error("Error logging role: {error}")]
    RoleLogError {
        #[from]
        error: RoleLogError,
        backtrace: Backtrace,
    },

    #[error("Error logging member: {error}")]
    MemberLogError {
        #[from]
        error: MemberLogError,
        backtrace: Backtrace,
    },

    #[error("Error logging ready event: {error}")]
    ReadEventError {
        #[from]
        error: ReadyEventError,
        backtrace: Backtrace,
    },

    #[error("Error logging user settings: {error}")]
    UserSettingsLogError {
        #[from]
        error: UserSettingsLogError,
        backtrace: Backtrace,
    },

    #[error("Error logging message clone: {error}")]
    MessageCloneLogError {
        #[from]
        error: MessageCloneLogError,
        backtrace: Backtrace,
    },

    #[error("Problem communicating with bot: {error}")]
    Bot {
        #[from]
        error: RouterError<BotServer>,
        backtrace: Backtrace,
    },
}
