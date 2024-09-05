use std::{error::Error as StdError, fmt::Display, io::Error as StdIoError, num::ParseIntError};

use h10::http::result::H10LibError;

use crate::proto::blockchain::result::BlockchainProtoError;

pub(crate) type ClientResult<T> = Result<T, ClientError>;

#[derive(Debug)]
pub(crate) enum ClientError {
    H10LibError(H10LibError),
    StdIoError(StdIoError),
    ParseIntError(ParseIntError),
    ParseAsciiHostname(String),
    NodeSigningKey(String),
    BlockchainProtoError(BlockchainProtoError),
    Custom(String),
}

impl ClientError {
    pub(crate) fn custom<S: ToString>(s: S) -> Self {
        Self::Custom(s.to_string())
    }
}

impl From<BlockchainProtoError> for ClientError {
    fn from(value: BlockchainProtoError) -> Self {
        Self::BlockchainProtoError(value)
    }
}

impl From<ParseIntError> for ClientError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl From<StdIoError> for ClientError {
    fn from(value: StdIoError) -> Self {
        Self::StdIoError(value)
    }
}

impl From<H10LibError> for ClientError {
    fn from(value: H10LibError) -> Self {
        Self::H10LibError(value)
    }
}
impl Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "".to_string();
        match self {
            Self::H10LibError(err) => output.push_str(format!("{err}").as_str()),
            Self::StdIoError(err) => output.push_str(format!("{err}").as_str()),
            Self::ParseIntError(err) => output.push_str(format!("{err}").as_str()),
            Self::ParseAsciiHostname(err) => output.push_str(format!("{err}").as_str()),
            Self::NodeSigningKey(err) => output.push_str(format!("{err}").as_str()),
            Self::BlockchainProtoError(err) => output.push_str(format!("{err}").as_str()),
            Self::Custom(err) => output.push_str(format!("{err}").as_str()),
        };
        write!(f, "{}", output)
    }
}

impl StdError for ClientError {}
