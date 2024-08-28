use std::{
    error::Error as StdError,
    fmt::Display,
    io::Error as StdIoError,
    net::AddrParseError,
    sync::{PoisonError, RwLockReadGuard},
};

use h10::http::{result::H10LibError, status_code::StatusCode};
use redb::{CommitError, DatabaseError, StorageError, TableError, TransactionError};

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
    DbTransactionError(TransactionError),
    DbTableError(TableError),
    DbDatabaseError(DatabaseError),
    DbStorageError(StorageError),
    DbCommitError(CommitError),
    Custom(String),
}

impl ServerError {
    pub(crate) fn custom<S: ToString>(s: S) -> Self {
        Self::Custom(s.to_string())
    }
}

impl From<CommitError> for ServerError {
    fn from(value: CommitError) -> Self {
        Self::DbCommitError(value)
    }
}

impl From<StorageError> for ServerError {
    fn from(value: StorageError) -> Self {
        Self::DbStorageError(value)
    }
}

impl From<DatabaseError> for ServerError {
    fn from(value: DatabaseError) -> Self {
        Self::DbDatabaseError(value)
    }
}

impl From<TableError> for ServerError {
    fn from(value: TableError) -> Self {
        Self::DbTableError(value)
    }
}

impl From<TransactionError> for ServerError {
    fn from(value: TransactionError) -> Self {
        Self::DbTransactionError(value)
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
            Self::H10LibError(err) => output.push_str(format!("{err}").as_str()),
            Self::StdIoError(err) => output.push_str(format!("{err}").as_str()),
            Self::AddrParseError(err) => output.push_str(format!("{err}").as_str()),
            Self::TomlFileError(err) => output.push_str(format!("{err}").as_str()),
            Self::PoisonErrorRwLockReadGuard => output.push_str("PoisonErrorRwLockReadGuard"),
            Self::PortParseError => output.push_str("PortParseError"),
            Self::InvalidLogLevel => output.push_str("Invalid LogLevel"),
            Self::DbTransactionError(err) => output.push_str(format!("{err}").as_str()),
            Self::DbTableError(err) => output.push_str(format!("{err}").as_str()),
            Self::DbDatabaseError(err) => output.push_str(format!("{err}").as_str()),
            Self::DbStorageError(err) => output.push_str(format!("{err}").as_str()),
            Self::DbCommitError(err) => output.push_str(format!("{err}").as_str()),
            Self::Custom(err) => output.push_str(format!("{err}").as_str()),
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
            | ServerError::DbTransactionError(_)
            | ServerError::DbTableError(_)
            | ServerError::DbDatabaseError(_)
            | ServerError::DbStorageError(_)
            | ServerError::DbCommitError(_)
            | ServerError::Custom(_) => StatusCode::InternalServerError,
        }
    }
}
