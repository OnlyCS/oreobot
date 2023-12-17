#![feature(error_generic_member_access)]
pub extern crate prisma_client_rust;

mod error;
#[allow(unused)]
#[rustfmt::skip]
mod generated;

pub mod prelude {
    pub use super::generated::{
        attachment, channel, channel_category, interaction, logless_roles, message, message_clone,
        role, user, user_settings_data, ChannelType, InteractionType, MessageCloneReason,
        PrismaClient,
    };

    pub mod prisma {
        use super::*;

        pub use prisma_client_rust::{and, not, or};

        pub async fn create() -> Result<PrismaClient, prisma_client_rust::NewClientError> {
            PrismaClient::_builder().build().await
        }

        pub mod data {
            pub use crate::generated::{
                attachment::Data as AttachmentData, channel::Data as ChannelData,
                channel_category::Data as ChannelCategoryData,
                interaction::Data as InteractionData, message::Data as MessageData,
                message_clone::Data as MessageCloneData, role::Data as RoleData,
                user::Data as UserData, user_settings_data::Data as UserSettingsData,
            };
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
