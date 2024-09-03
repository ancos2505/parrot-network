use crate::http::headers::{HeaderEntry, HeaderName, HeaderValue, IntoHeader};

/// ### Expires
/// Related: Resource state
///
///  The Expires entity-header field gives the date/time after which the entity
/// should be considered stale.
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#section-10.7
///
#[derive(Debug, PartialEq, Eq)]
pub struct Expires {
    name: HeaderName,
    value: HeaderValue,
}

impl Default for Expires {
    fn default() -> Self {
        Self {
            name: HeaderName::new_unchecked("Expires"),
            value: HeaderValue::new_unchecked("Not_Defined"),
        }
    }
}

impl IntoHeader for Expires {
    fn into_header(self) -> HeaderEntry {
        let Self { name, value } = self;
        HeaderEntry { name, value }
    }
}
