#[allow(warnings, unused)]
pub mod prisma_client;

pub use prisma_client::new_client as create;
pub use prisma_client::PrismaClient;
