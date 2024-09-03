use std::{
    array::TryFromSliceError, error::Error as StdError, fmt::Display, num::TryFromIntError,
    time::SystemTimeError,
};

use array_bytes::Error as ArrayBytesError;

pub(crate) type BlockchainProtoResult<T> = Result<T, BlockchainProtoError>;

#[derive(Debug)]
pub(crate) enum BlockchainProtoError {
    TryFromSliceError(TryFromSliceError),
    TryFromIntError(TryFromIntError),
    SystemTimeError(SystemTimeError),
    ArrayBytesError(ArrayBytesError),
    TokenConversion(String),
    FalconDeserializationError(String),
    Custom(String),
}

impl BlockchainProtoError {
    pub(crate) fn custom(s: &str) -> Self {
        Self::Custom(s.into())
    }
}

impl From<ArrayBytesError> for BlockchainProtoError {
    fn from(value: ArrayBytesError) -> Self {
        Self::ArrayBytesError(value)
    }
}

impl From<SystemTimeError> for BlockchainProtoError {
    fn from(value: SystemTimeError) -> Self {
        Self::SystemTimeError(value)
    }
}

impl From<TryFromIntError> for BlockchainProtoError {
    fn from(value: TryFromIntError) -> Self {
        Self::TryFromIntError(value)
    }
}

impl From<TryFromSliceError> for BlockchainProtoError {
    fn from(value: TryFromSliceError) -> Self {
        Self::TryFromSliceError(value)
    }
}

impl Display for BlockchainProtoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "".to_string();
        match self {
            BlockchainProtoError::TryFromSliceError(err) => {
                output.push_str(format!("{err}").as_str())
            }
            BlockchainProtoError::TryFromIntError(err) => {
                output.push_str(format!("{err}").as_str())
            }
            BlockchainProtoError::SystemTimeError(err) => {
                output.push_str(format!("{err}").as_str())
            }
            BlockchainProtoError::ArrayBytesError(err) => {
                output.push_str(format!("{err:?}").as_str())
            }
            BlockchainProtoError::TokenConversion(err) => {
                output.push_str(format!("{err}").as_str())
            }
            BlockchainProtoError::FalconDeserializationError(err) => {
                output.push_str(format!("{err}").as_str())
            }
            BlockchainProtoError::Custom(err) => output.push_str(format!("{err}").as_str()),
        };
        write!(f, "{}", output)
    }
}

impl StdError for BlockchainProtoError {}
