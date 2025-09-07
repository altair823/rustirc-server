use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Invalid message format")]
    InvalidFormat,
    #[error("Unknown command")]
    UnknownCommand,
    #[error("Invalid prefix")]
    InvalidPrefix,
    #[error("Invalid parameters")]
    InvalidParameters,
}
