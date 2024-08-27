use std::array::TryFromSliceError;

use ed25519_dalek::SignatureError;

pub type H10BlockchainProtoResult<T> = Result<T, H10BlockchainProtoError>;

#[derive(Debug)]
pub enum H10BlockchainProtoError {
    SignatureError(SignatureError),
    TryFromSliceError(TryFromSliceError),
    Custom(String),
}

impl From<TryFromSliceError> for H10BlockchainProtoError {
    fn from(value: TryFromSliceError) -> Self {
        Self::TryFromSliceError(value)
    }
}

impl From<SignatureError> for H10BlockchainProtoError {
    fn from(value: SignatureError) -> Self {
        Self::SignatureError(value)
    }
}
