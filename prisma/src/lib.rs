#![feature(error_generic_member_access)]
pub extern crate prisma_client_rust;
mod error;
#[allow(unused)]
mod generated;

pub mod prelude {
    pub use super::generated::{
        attachment, channel, channel_category, interaction, message, message_pin, news_in_chat,
        role, user, user_settings_data, ChannelType, InteractionType, PrismaClient,
    };

    pub mod prisma {
        use super::*;

        pub use prisma_client_rust::{and, not, or};

        pub async fn create() -> Result<PrismaClient, prisma_client_rust::NewClientError> {
            PrismaClient::_builder().build().await
        }
    }

    pub mod prisma_error {
        pub use crate::error::*;
    }

    pub trait DatabaseId {
        fn database_id(&self) -> i64;
    }

    impl<T: Into<u64> + Copy> DatabaseId for T {
        fn database_id(&self) -> i64 {
            Into::<u64>::into(*self) as i64
        }
    }
}
