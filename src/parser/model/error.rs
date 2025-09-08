use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ParseError {
    #[error("Invalid message format")]
    InvalidFormat,
    #[error("Unknown command")]
    UnknownCommand,
    #[error("Invalid prefix: `{0}`")]
    InvalidPrefix(String),
    #[error("Invalid parameters")]
    InvalidParameters,
}
