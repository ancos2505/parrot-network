use std::{error::Error as StdError, fmt::Display, io::Error as StdIoError, num::ParseIntError};

use h10::http::result::H10LibError;

pub(crate) type WebUiResult<T> = Result<T, WebUiError>;

#[derive(Debug)]
pub(crate) enum WebUiError {
    H10LibError(H10LibError),
    StdIoError(StdIoError),
    ParseIntError(ParseIntError),
    // AddrParseError(AddrParseError),
    // TomlFileError(toml::de::Error),
    // PoisonErrorRwLockReadGuard,
    // PortParseError,
    // InvalidLogLevel,
    // DbTransactionError(TransactionError),
    // DbTableError(TableError),
    // DbDatabaseError(DatabaseError),
    // DbStorageError(StorageError),
    // DbCommitError(CommitError),
    // SerdeJson(SerdeJsonError),
    // SystemTimeError(SystemTimeError),
    // TryFromIntError(TryFromIntError),
    // TryFromSliceError(TryFromSliceError),
    // BlockchainProtoError(BlockchainProtoError),
    // Ed25519Error(Ed25519Error),
    Custom(String),
}

impl WebUiError {
    pub(crate) fn custom<S: ToString>(s: S) -> Self {
        Self::Custom(s.to_string())
    }
}

impl From<ParseIntError> for WebUiError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}
// impl From<BlockchainProtoError> for WebUiError {
//     fn from(value: BlockchainProtoError) -> Self {
//         Self::BlockchainProtoError(value)
//     }
// }
// impl From<Ed25519Error> for WebUiError {
//     fn from(value: Ed25519Error) -> Self {
//         Self::Ed25519Error(value)
//     }
// }

// // impl From<TryFromSliceError> for WebUiError {
// //     fn from(value: TryFromSliceError) -> Self {
// //         Self::TryFromSliceError(value)
// //     }
// // }

// // impl From<TryFromIntError> for WebUiError {
// //     fn from(value: TryFromIntError) -> Self {
// //         Self::TryFromIntError(value)
// //     }
// // }
// // impl From<SystemTimeError> for WebUiError {
// //     fn from(value: SystemTimeError) -> Self {
// //         Self::SystemTimeError(value)
// //     }
// // }

// // impl From<SerdeJsonError> for WebUiError {
// //     fn from(value: SerdeJsonError) -> Self {
// //         Self::SerdeJson(value)
// //     }
// // }

// impl From<CommitError> for WebUiError {
//     fn from(value: CommitError) -> Self {
//         Self::DbCommitError(value)
//     }
// }

// impl From<StorageError> for WebUiError {
//     fn from(value: StorageError) -> Self {
//         Self::DbStorageError(value)
//     }
// }

// impl From<DatabaseError> for WebUiError {
//     fn from(value: DatabaseError) -> Self {
//         Self::DbDatabaseError(value)
//     }
// }

// impl From<TableError> for WebUiError {
//     fn from(value: TableError) -> Self {
//         Self::DbTableError(value)
//     }
// }

// impl From<TransactionError> for WebUiError {
//     fn from(value: TransactionError) -> Self {
//         Self::DbTransactionError(value)
//     }
// }

// impl From<toml::de::Error> for WebUiError {
//     fn from(value: toml::de::Error) -> Self {
//         Self::TomlFileError(value)
//     }
// }
// impl<T> From<PoisonError<RwLockReadGuard<'_, T>>> for WebUiError {
//     fn from(_: PoisonError<RwLockReadGuard<'_, T>>) -> Self {
//         Self::PoisonErrorRwLockReadGuard
//     }
// }
// impl From<AddrParseError> for WebUiError {
//     fn from(value: AddrParseError) -> Self {
//         Self::AddrParseError(value)
//     }
// }
impl From<StdIoError> for WebUiError {
    fn from(value: StdIoError) -> Self {
        Self::StdIoError(value)
    }
}

impl From<H10LibError> for WebUiError {
    fn from(value: H10LibError) -> Self {
        Self::H10LibError(value)
    }
}
impl Display for WebUiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "".to_string();
        match self {
            Self::H10LibError(err) => output.push_str(format!("{err}").as_str()),
            Self::StdIoError(err) => output.push_str(format!("{err}").as_str()),
            Self::ParseIntError(err) => output.push_str(format!("{err}").as_str()),
            // Self::AddrParseError(err) => output.push_str(format!("{err}").as_str()),
            // Self::TomlFileError(err) => output.push_str(format!("{err}").as_str()),
            // Self::PoisonErrorRwLockReadGuard => output.push_str("PoisonErrorRwLockReadGuard"),
            // Self::PortParseError => output.push_str("PortParseError"),
            // Self::InvalidLogLevel => output.push_str("Invalid LogLevel"),
            // Self::DbTransactionError(err) => output.push_str(format!("{err}").as_str()),
            // Self::DbTableError(err) => output.push_str(format!("{err}").as_str()),
            // Self::DbDatabaseError(err) => output.push_str(format!("{err}").as_str()),
            // Self::DbStorageError(err) => output.push_str(format!("{err}").as_str()),
            // Self::DbCommitError(err) => output.push_str(format!("{err}").as_str()),
            // Self::SerdeJson(err) => output.push_str(format!("{err}").as_str()),
            // Self::SystemTimeError(err) => output.push_str(format!("{err}").as_str()),
            // Self::TryFromIntError(err) => output.push_str(format!("{err}").as_str()),
            // Self::TryFromSliceError(err) => output.push_str(format!("{err}").as_str()),
            // Self::Ed25519Error(err) => output.push_str(format!("{err}").as_str()),
            // Self::BlockchainProtoError(err) => output.push_str(format!("{err}").as_str()),
            Self::Custom(err) => output.push_str(format!("{err}").as_str()),
        };
        write!(f, "{}", output)
    }
}

impl StdError for WebUiError {}
