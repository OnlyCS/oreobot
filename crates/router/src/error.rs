use thiserror::Error;

#[derive(Error, Debug)]
pub enum RouterError {
    #[error("Error with serde: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("Error with tokio: {0}")]
    TokioError(#[from] tokio::io::Error),

    #[error("Read error: {0}")]
    ReadError(#[from] std::string::FromUtf8Error),

    #[error("Error parsing message length: {0}")]
    ParseError(#[from] std::num::ParseIntError),
}
