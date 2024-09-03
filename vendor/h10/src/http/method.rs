use std::{fmt::Display, str::FromStr};

use crate::http::result::H10LibError;

/// ### HTTP Method
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#section-5.1.1
///
/// Aditional methods: https://www.rfc-editor.org/rfc/rfc1945.html#appendix-D.1
#[derive(Debug, PartialEq, Eq)]
pub enum Method {
    Get,
    Head,
    Post,
    // Aditional methods,
    Put,
    Delete,
    Link,
    Unlink,
}

impl Method {
    pub const MAX_LENGTH: usize = 6;

    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Get => "GET",
            Self::Head => "HEAD",
            Self::Post => "POST",
            Self::Put => "PUT",
            Self::Delete => "DELETE",
            Self::Link => "LINK",
            Self::Unlink => "UNLINK",
        }
    }
}
impl FromStr for Method {
    type Err = H10LibError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let method = match s {
            "GET" => Self::Get,
            "HEAD" => Self::Head,
            "POST" => Self::Post,
            "PUT" => Self::Put,
            "DELETE" => Self::Delete,
            "LINK" => Self::Link,
            "UNLINK" => Self::Unlink,
            _ => return Err(H10LibError::MethodNotSupported),
        };
        Ok(method)
    }
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
