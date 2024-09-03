use crate::http::{
    headers::{HeaderEntry, HeaderName, HeaderValue, IntoHeader},
    result::H10LibResult,
};

/// ### WWW-Authenticate header
/// Related: Authentication/Authorization/Session
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#section-10.16
#[derive(Debug, PartialEq, Eq)]
pub struct WWWAuthenticate {
    name: HeaderName,
    value: HeaderValue,
}

impl Default for WWWAuthenticate {
    fn default() -> Self {
        Self {
            name: HeaderName::new_unchecked("WWW-Authenticate"),
            value: HeaderValue::new_unchecked("Not_Defined"),
        }
    }
}

impl WWWAuthenticate {
    pub fn new(new_value: &str) -> H10LibResult<Self> {
        Ok(Self {
            value: new_value.parse()?,
            ..Default::default()
        })
    }
}

impl IntoHeader for WWWAuthenticate {
    fn into_header(self) -> HeaderEntry {
        let Self { name, value } = self;
        HeaderEntry { name, value }
    }
}
