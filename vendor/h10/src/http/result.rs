use std::{
    error::Error,
    fmt::Display,
    io::Error as IoError,
    num::{ParseFloatError, ParseIntError},
    str::Utf8Error,
    string::FromUtf8Error,
    sync::mpsc::SendError,
    time::SystemTimeError,
};

use super::status_code::StatusCode;

pub type H10LibResult<T> = Result<T, H10LibError>;

#[derive(Debug)]
pub enum H10LibError {
    VersionNotSupported,
    MethodNotSupported,
    StatusCodeNotSupported,
    InvalidInputData(String),
    ParseFloatError(ParseFloatError),
    SystemTimeError(SystemTimeError),
    ParseIntError(ParseIntError),
    FromUtf8Error,
    Utf8Error,
    RequestParser(String),
    HeadersParser(String),
    QueryStringParser(String),
    ResponseParser(String),
    IoError(IoError),
    StrFromAscii(String),
    ClientError(H10ClientError),
    MpscSendError,
    Custom(String),
}

impl H10LibError {
    pub fn custom(msg: &str) -> Self {
        Self::Custom(msg.to_owned())
    }
}

#[derive(Debug)]
pub enum H10ClientError {
    Timeout,
    InternalError(String),
}

impl<T: ToString> From<SendError<T>> for H10LibError {
    fn from(_: SendError<T>) -> Self {
        Self::MpscSendError
    }
}

impl From<FromUtf8Error> for H10LibError {
    fn from(_: FromUtf8Error) -> Self {
        Self::FromUtf8Error
    }
}

impl From<Utf8Error> for H10LibError {
    fn from(_: Utf8Error) -> Self {
        Self::Utf8Error
    }
}

impl From<ParseFloatError> for H10LibError {
    fn from(error: ParseFloatError) -> Self {
        Self::ParseFloatError(error)
    }
}

impl From<SystemTimeError> for H10LibError {
    fn from(error: SystemTimeError) -> Self {
        Self::SystemTimeError(error)
    }
}

impl From<ParseIntError> for H10LibError {
    fn from(error: ParseIntError) -> Self {
        Self::ParseIntError(error)
    }
}

impl From<IoError> for H10LibError {
    fn from(error: IoError) -> Self {
        Self::IoError(error)
    }
}

impl Display for H10LibError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for H10LibError {}

impl From<H10LibError> for StatusCode {
    fn from(value: H10LibError) -> Self {
        match value {
            H10LibError::VersionNotSupported
            | H10LibError::StatusCodeNotSupported
            | H10LibError::MethodNotSupported
            | H10LibError::RequestParser(_)
            | H10LibError::HeadersParser(_)
            | H10LibError::QueryStringParser(_)
            | H10LibError::ResponseParser(_)
            | H10LibError::InvalidInputData(_)
            | H10LibError::StrFromAscii(_)
            | H10LibError::Utf8Error => StatusCode::BadRequest,
            H10LibError::ParseFloatError(_)
            | H10LibError::SystemTimeError(_)
            | H10LibError::ParseIntError(_)
            | H10LibError::IoError(_)
            | H10LibError::Custom(_)
            | H10LibError::ClientError(_)
            | H10LibError::MpscSendError
            | H10LibError::FromUtf8Error => StatusCode::InternalServerError,
        }
    }
}
