use oauth2::{
    RequestTokenError,
    StandardErrorResponse,
    basic::BasicErrorResponseType,
};
use crate::EnvironmentVariable;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    // Error variants originating from this crate
    RequestTokenError,
    NoIncomingStreams,
    MissingEnvironmentVariable(EnvironmentVariable),

    // Wrappers for other errors
    IOError(std::io::Error),
    ReqwestError(reqwest::Error),
    SerdeJsonError(serde_json::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RequestTokenError => f.write_str("RequestTokenError"),
            Self::NoIncomingStreams => f.write_str("NoIncomingStreams"),
            Self::MissingEnvironmentVariable(v) => f.write_str(&format!("Missing environment variable {v}")),

            Self::IOError(e) => e.fmt(f),
            Self::ReqwestError(e) => e.fmt(f),
            Self::SerdeJsonError(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IOError(err)
    }
}

impl From<RequestTokenError<oauth2::reqwest::Error<reqwest::Error>, StandardErrorResponse<BasicErrorResponseType>>> for Error {
    fn from(_err: RequestTokenError<oauth2::reqwest::Error<reqwest::Error>, StandardErrorResponse<BasicErrorResponseType>>) -> Self {
        Self::RequestTokenError
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::ReqwestError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::SerdeJsonError(err)
    }
}

