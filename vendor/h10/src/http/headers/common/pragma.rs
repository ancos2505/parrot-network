use crate::http::headers::{HeaderEntry, HeaderName, HeaderValue, IntoHeader};

/// ### Pragma - header
/// Related: Content state
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#section-10.12
#[derive(Debug, PartialEq, Eq)]
pub struct Pragma {
    name: HeaderName,
    value: HeaderValue,
}
impl Default for Pragma {
    fn default() -> Self {
        Self {
            name: HeaderName::new_unchecked("Pragma"),
            value: HeaderValue::new_unchecked("no-cache"),
        }
    }
}

impl IntoHeader for Pragma {
    fn into_header(self) -> HeaderEntry {
        let Self { name, value } = self;
        HeaderEntry { name, value }
    }
}
