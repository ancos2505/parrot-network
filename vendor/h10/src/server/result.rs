use std::{
    error::Error as StdError,
    fmt::Display,
    io::Error as StdIoError,
    net::AddrParseError,
    sync::{PoisonError, RwLockReadGuard},
};

use h10::http::result::H10LibError;

pub(crate) type ServerResult<T> = Result<T, ServerError>;

#[derive(Debug)]
pub(crate) enum ServerError {
    // H10LibError(H10LibError),
    StdIoError(String),
    AddrParseError(String),
    PoisonErrorRwLockReadGuard(String),
    PortParseError(String),
    InvalidLogLevel(String),
    InvalidCLiArgs(String),
    Custom(String),
}
impl ServerError {
    pub(crate) fn custom<S: ToString>(s: S) -> Self {
        Self::Custom(s.to_string())
    }
}

impl<T> From<PoisonError<RwLockReadGuard<'_, T>>> for ServerError {
    fn from(err: PoisonError<RwLockReadGuard<'_, T>>) -> Self {
        Self::PoisonErrorRwLockReadGuard(err.to_string())
    }
}

impl From<AddrParseError> for ServerError {
    fn from(err: AddrParseError) -> Self {
        Self::AddrParseError(err.to_string())
    }
}

impl From<StdIoError> for ServerError {
    fn from(err: StdIoError) -> Self {
        Self::StdIoError(err.to_string())
    }
}

// impl From<H10LibError> for ServerError {
//     fn from(value: H10LibError) -> Self {
//         Self::H10LibError(value)
//     }
// }

impl Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl StdError for ServerError {}
