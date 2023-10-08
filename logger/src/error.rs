use crate::prelude::*;

use std::backtrace::Backtrace;

#[derive(Error, Debug)]
pub enum MessageLogError {
    #[error("Warning: Message ({{ id: {0} }}) is owned by a webhook, skipping")]
    WebhookMessage(serenity::MessageId),

    #[error("Warning: Message ({{ id: {0} }}) is in the #news channel, skipping")]
    NewsMessage(serenity::MessageId),

    #[error("Warning: User ({{ id: {0} }}) is impersonating another user, skipping")]
    UserImpersonated(serenity::UserId),

    #[error("Message ({{ id: {0} }}) not found")]
    MessageNotFound(serenity::MessageId),

    #[error("Warning: Skipping delete for message ({{ id: {0} }}), impersonated")]
    MessageImpersonated(serenity::MessageId),

    #[error("Problem with database: {error}")]
    Database {
        #[from]
        error: prisma_error::PrismaError,
        backtrace: Backtrace,
    },
}

#[derive(Error, Debug)]
pub enum CategoryLogError {
    #[error("Problem with database: {error}")]
    Database {
        #[from]
        error: prisma_error::PrismaError,
        backtrace: Backtrace,
    },
}

#[derive(Error, Debug)]
pub enum ChannelLogError {
    #[error("Warning: Channel ({{ id: {0} }}) is a thread, skipping")]
    Thread(serenity::ChannelId),

    #[error("Problem with database: {error}")]
    Database {
        #[from]
        error: prisma_error::PrismaError,
        backtrace: Backtrace,
    },
}

#[derive(Error, Debug)]
pub enum InteractionLogError {
    #[error("Problem with database: {error}")]
    Database {
        #[from]
        error: prisma_error::PrismaError,
        backtrace: Backtrace,
    },
}

#[derive(Error, Debug)]
pub enum MemberLogError {
    #[error("Problem with database: {error}")]
    Database {
        #[from]
        error: prisma_error::PrismaError,
        backtrace: Backtrace,
    },

    #[error("Member ({{ id: {0} }}) not found")]
    MemberNotFound(serenity::UserId),

    #[error("Member ({{ id: {0} }}) has no color role")]
    NoColorRole(serenity::UserId),
}

#[derive(Error, Debug)]
pub enum RoleLogError {
    #[error("Problem with database: {error}")]
    Database {
        #[from]
        error: prisma_error::PrismaError,
        backtrace: Backtrace,
    },

    #[error("Role ({{ id: {0} }}) is blacklisted")]
    Blacklisted(serenity::RoleId),

    #[error("Role ({{ id: {0} }}) is a custom role")]
    CustomRole(serenity::RoleId),

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
    #[error("Problem with database: {error}")]
    Database {
        #[from]
        error: prisma_error::PrismaError,
        backtrace: Backtrace,
    },

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

prisma_error_convert!(
    CategoryLogError,
    MessageLogError,
    ChannelLogError,
    InteractionLogError,
    MemberLogError,
    RoleLogError
);
