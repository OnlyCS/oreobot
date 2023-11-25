use prisma_client_rust::{NewClientError, QueryError, RelationNotFetchedError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PrismaError {
    #[error("Failed to create Prisma client: {0}")]
    NewClient(#[from] NewClientError),

    #[error("Failed to execute query: {0}")]
    Query(#[from] QueryError),

    #[error("Failed to fetch relation: {0}")]
    RelationNotFetched(#[from] RelationNotFetchedError),
}

#[macro_export]
macro_rules! prisma_error_convert {
    ($($err:ty),*) => {
        $(
            impl From<prisma_client_rust::NewClientError> for $err {
                fn from(value: prisma_client_rust::NewClientError) -> Self {
                    Self::from(prisma_error::PrismaError::from(value))
                }
            }

            impl From<prisma_client_rust::QueryError> for $err {
                fn from(value: prisma_client_rust::QueryError) -> Self {
                    Self::from(prisma_error::PrismaError::from(value))
                }
            }

            impl From<prisma_client_rust::RelationNotFetchedError> for $err {
                fn from(value: prisma_client_rust::RelationNotFetchedError) -> Self {
                    Self::from(prisma_error::PrismaError::from(value))
                }
            }
        )*
    };
}
