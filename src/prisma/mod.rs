#[allow(warnings, unused)]
pub mod prisma_client;

pub use prisma_client::new_client as create;
pub use prisma_client::PrismaClient;

use crate::prelude::*;

pub struct PrismaTypeKey;

impl serenity::TypeMapKey for PrismaTypeKey {
    type Value = Arc<Mutex<PrismaClient>>;
}
