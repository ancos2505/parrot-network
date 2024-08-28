use std::{
    error::Error as StdError,
    fmt::Display,
    io::Error as StdIoError,
    net::AddrParseError,
    sync::{PoisonError, RwLockReadGuard},
};

use h10::http::{result::H10LibError, status_code::StatusCode};

pub(crate) type ServerResult<T> = Result<T, ServerError>;

#[derive(Debug)]
pub(crate) enum ServerError {
    H10LibError(H10LibError),
    StdIoError(StdIoError),
    AddrParseError(AddrParseError),
    TomlFileError(toml::de::Error),
    PoisonErrorRwLockReadGuard,
    PortParseError,
    InvalidLogLevel,
    Custom(String),
}
impl ServerError {
    pub(crate) fn custom<S: ToString>(s: S) -> Self {
        Self::Custom(s.to_string())
    }
}

impl From<toml::de::Error> for ServerError {
    fn from(value: toml::de::Error) -> Self {
        Self::TomlFileError(value)
    }
}
impl<T> From<PoisonError<RwLockReadGuard<'_, T>>> for ServerError {
    fn from(_: PoisonError<RwLockReadGuard<'_, T>>) -> Self {
        Self::PoisonErrorRwLockReadGuard
    }
}
impl From<AddrParseError> for ServerError {
    fn from(value: AddrParseError) -> Self {
        Self::AddrParseError(value)
    }
}
impl From<StdIoError> for ServerError {
    fn from(value: StdIoError) -> Self {
        Self::StdIoError(value)
    }
}
impl From<H10LibError> for ServerError {
    fn from(value: H10LibError) -> Self {
        Self::H10LibError(value)
    }
}
impl Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "".to_string();
        match self {
            ServerError::H10LibError(err) => output.push_str(format!("{err}").as_str()),
            ServerError::StdIoError(err) => output.push_str(format!("{err}").as_str()),
            ServerError::AddrParseError(err) => output.push_str(format!("{err}").as_str()),
            ServerError::TomlFileError(err) => output.push_str(format!("{err}").as_str()),
            ServerError::PoisonErrorRwLockReadGuard => {
                output.push_str("PoisonErrorRwLockReadGuard")
            }
            ServerError::PortParseError => output.push_str("PortParseError"),
            ServerError::InvalidLogLevel => output.push_str("Invalid LogLevel"),
            ServerError::Custom(err) => output.push_str(format!("{err}").as_str()),
        };
        write!(f, "{}", output)
    }
}

impl StdError for ServerError {}

impl From<ServerError> for StatusCode {
    fn from(value: ServerError) -> Self {
        match value {
            ServerError::H10LibError(h10error) => h10error.into(),
            ServerError::StdIoError(_)
            | ServerError::AddrParseError(_)
            | ServerError::PoisonErrorRwLockReadGuard
            | ServerError::PortParseError
            | ServerError::InvalidLogLevel
            | ServerError::TomlFileError(_)
            | ServerError::Custom(_) => StatusCode::InternalServerError,
        }
    }
}
