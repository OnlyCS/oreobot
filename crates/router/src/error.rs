use crate::prelude::*;

#[derive(Error, Debug)]
pub enum RouterError<Meta: ServerMetadata> {
    #[error("Error with serde: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("Error with tokio: {0}")]
    TokioError(#[from] tokio::io::Error),

    #[error("Read error: {0}")]
    ReadError(#[from] std::string::FromUtf8Error),

    #[error("Error parsing message length: {0}")]
    ParseError(#[from] std::num::ParseIntError),

    #[error("Error with server: {0}")]
    ServerError(Meta::Error),

    #[error("Server not ready")]
    ServerNotReady,

    #[error("Invalid response from server")]
    InvalidResponse,
}
