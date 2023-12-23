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
