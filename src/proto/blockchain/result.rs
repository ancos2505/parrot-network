use std::{
    array::TryFromSliceError, error::Error as StdError, fmt::Display, num::TryFromIntError,
    time::SystemTimeError,
};

use ed25519_dalek::SignatureError;

pub(crate) type BlockchainProtoResult<T> = Result<T, BlockchainProtoError>;

#[derive(Debug)]
pub(crate) enum BlockchainProtoError {
    SignatureError(SignatureError),
    TryFromSliceError(TryFromSliceError),
    TryFromIntError(TryFromIntError),
    SystemTimeError(SystemTimeError),
    TokenConversion(String),
    Custom(String),
}

impl BlockchainProtoError {
    pub(crate) fn custom(s: &str) -> Self {
        Self::Custom(s.into())
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

impl From<SignatureError> for BlockchainProtoError {
    fn from(value: SignatureError) -> Self {
        Self::SignatureError(value)
    }
}

impl Display for BlockchainProtoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "".to_string();
        match self {
            Self::SignatureError(err) => output.push_str(format!("{err}").as_str()),
            Self::TryFromSliceError(err) => output.push_str(format!("{err}").as_str()),
            Self::TryFromIntError(err) => output.push_str(format!("{err}").as_str()),
            Self::SystemTimeError(err) => output.push_str(format!("{err}").as_str()),
            Self::TokenConversion(err) => output.push_str(format!("{err}").as_str()),
            Self::Custom(err) => output.push_str(format!("{err}").as_str()),
        };
        write!(f, "{}", output)
    }
}

impl StdError for BlockchainProtoError {}
