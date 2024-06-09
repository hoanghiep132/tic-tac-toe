use std::num::ParseIntError;

use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Ord, PartialOrd, Eq, Hash, Error)]
#[allow(missing_docs)]
pub enum AppError {
    #[error("unknown error")]
    InternalError(String),

    #[error("Wrong password.")]
    AuthFailed,

    #[error("Unable to connect to destination server.")]
    ConnectionError(String),

    #[error("{0:?}")]
    ApiError(String),

    #[error("")]
    CodeParseError(),

    #[error("Didnt get any digits")]
    NoInput,
}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        Self::InternalError(error.to_string())
    }
}

impl From<tokio::sync::oneshot::error::RecvError> for AppError {
    fn from(error: tokio::sync::oneshot::error::RecvError) -> Self {
        Self::InternalError(error.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(error: serde_json::Error) -> Self {
        Self::InternalError(error.to_string())
    }
}

impl From<ParseIntError> for AppError {
    fn from(error: ParseIntError) -> Self {
        Self::InternalError(error.to_string())
    }
}
