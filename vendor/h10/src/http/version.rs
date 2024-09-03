use std::{fmt::Display, str::FromStr};

use crate::http::result::H10LibError;

#[derive(Debug, PartialEq, Eq)]
pub enum Version {
    Http1_0,
    Http1_1,
}

impl Version {
    pub const MAX_LENGTH: usize = 8;
    pub const fn as_str(&self) -> &'static str {
        match self {
            Version::Http1_0 => "HTTP/1.0",
            Version::Http1_1 => "HTTP/1.1",
        }
    }
}
impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for Version {
    type Err = H10LibError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HTTP/1.0" => Ok(Self::Http1_0),
            "HTTP/1.1" => Ok(Self::Http1_1),
            _ => Err(H10LibError::VersionNotSupported),
        }
    }
}
