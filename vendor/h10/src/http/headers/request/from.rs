use crate::http::{
    headers::{HeaderEntry, HeaderName, HeaderValue, IntoHeader},
    result::H10LibResult,
};

/// ### From
/// Related: Public Key Infrastructure
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#section-10.8
#[derive(Debug, PartialEq, Eq)]
pub struct From {
    name: HeaderName,
    value: HeaderValue,
}

impl Default for From {
    fn default() -> Self {
        Self {
            name: HeaderName::new_unchecked("From"),
            value: HeaderValue::new_unchecked("Not_Defined"),
        }
    }
}

impl From {
    pub fn new(new_value: &str) -> H10LibResult<Self> {
        Ok(Self {
            value: new_value.parse()?,
            ..Default::default()
        })
    }
}

impl IntoHeader for From {
    fn into_header(self) -> HeaderEntry {
        let Self { name, value } = self;
        HeaderEntry { name, value }
    }
}
