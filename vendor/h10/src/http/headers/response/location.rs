use crate::http::{
    headers::{HeaderEntry, HeaderName, HeaderValue, IntoHeader},
    result::H10LibResult,
    url_path::UrlPath,
};

/// ### Location header
/// Related: Indicate a redirection
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#section-10.11
#[derive(Debug, PartialEq, Eq)]
pub struct Location {
    name: HeaderName,
    value: HeaderValue,
}

impl Default for Location {
    fn default() -> Self {
        Self {
            name: HeaderName::new_unchecked("Location"),
            value: HeaderValue::new_unchecked("Not_Defined"),
        }
    }
}

impl Location {
    pub fn from_str<S: AsRef<str>>(url: S) -> H10LibResult<Self> {
        Ok(Self {
            value: HeaderValue::new_unchecked(UrlPath::parse(url.as_ref())?.to_string().as_str()),
            ..Default::default()
        })
    }
}

impl IntoHeader for Location {
    fn into_header(self) -> HeaderEntry {
        let Self { name, value } = self;
        HeaderEntry { name, value }
    }
}
