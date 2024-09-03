use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use super::result::H10LibError;

#[derive(Debug, PartialEq, Eq)]
pub enum StatusCode {
    OK,
    Created,
    Accepted,
    NoContent,
    MovedPermanently,
    MovedTemporarily,
    NotModified,
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
}
impl StatusCode {
    pub const MAX_LENGTH: usize = 3;
}

impl FromStr for StatusCode {
    type Err = H10LibError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let code: u16 = s.parse()?;
        code.try_into()
    }
}

impl TryFrom<u16> for StatusCode {
    type Error = H10LibError;

    fn try_from(code: u16) -> Result<Self, Self::Error> {
        let status_code = match code {
            200 => StatusCode::OK,
            201 => StatusCode::Created,
            202 => StatusCode::Accepted,
            204 => StatusCode::NoContent,
            301 => StatusCode::MovedPermanently,
            302 => StatusCode::MovedTemporarily,
            304 => StatusCode::NotModified,
            400 => StatusCode::BadRequest,
            401 => StatusCode::Unauthorized,
            403 => StatusCode::Forbidden,
            404 => StatusCode::NotFound,
            500 => StatusCode::InternalServerError,
            501 => StatusCode::NotImplemented,
            502 => StatusCode::BadGateway,
            503 => StatusCode::ServiceUnavailable,
            _ => {
                return Err(H10LibError::custom(
                    format!("Invalid code number for {}", stringify!(Self)).as_str(),
                ))
            }
        };
        Ok(status_code)
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            StatusCode::OK => "200 OK",
            StatusCode::Created => "201 Created",
            StatusCode::Accepted => "202 Accepted",
            StatusCode::NoContent => "204 No Content",
            StatusCode::MovedPermanently => "301 Moved Permanently",
            StatusCode::MovedTemporarily => "302 Moved Temporarily",
            StatusCode::NotModified => "304 Not Modified",
            StatusCode::BadRequest => "400 Bad Request",
            StatusCode::Unauthorized => "401 Unauthorized",
            StatusCode::Forbidden => "403 Forbidden",
            StatusCode::NotFound => "404 Not Found",
            StatusCode::InternalServerError => "500 Internal Server Error",
            StatusCode::NotImplemented => "501 Not Implemented",
            StatusCode::BadGateway => "502 Bad Gateway",
            StatusCode::ServiceUnavailable => "503 Service Unavailable",
        };

        write!(f, "{}", output)
    }
}
