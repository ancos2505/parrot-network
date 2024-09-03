use crate::http::headers::{HeaderEntry, HeaderName, HeaderValue, IntoHeader};

/// ### Title
/// Related: Content handling
///
///  The Title entity-header field indicates the title of the entity.
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#appendix-D.2.6
///
#[derive(Debug, PartialEq, Eq)]
pub struct Title {
    name: HeaderName,
    value: HeaderValue,
}

impl Default for Title {
    fn default() -> Self {
        Self {
            name: HeaderName::new_unchecked("Title"),
            value: HeaderValue::new_unchecked("Not_Defined"),
        }
    }
}

impl IntoHeader for Title {
    fn into_header(self) -> HeaderEntry {
        let Self { name, value } = self;
        HeaderEntry { name, value }
    }
}
