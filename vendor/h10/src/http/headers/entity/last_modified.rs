use crate::http::headers::{HeaderEntry, HeaderName, HeaderValue, IntoHeader};

/// ### Last-Modified
/// Related: Content state
///
///  The Last-Modified entity-header field indicates the date and time at which
/// the sender believes the resource was last modified.
///
/// **Reference:** https://www.rfc-editor.org/rfc/rfc1945.html#section-10.10
///
#[derive(Debug, PartialEq, Eq)]
pub struct LastModified {
    name: HeaderName,
    value: HeaderValue,
}

impl Default for LastModified {
    fn default() -> Self {
        Self {
            name: HeaderName::new_unchecked("Last-Modified"),
            value: HeaderValue::new_unchecked("Not_Defined"),
        }
    }
}

impl IntoHeader for LastModified {
    fn into_header(self) -> HeaderEntry {
        let Self { name, value } = self;
        HeaderEntry { name, value }
    }
}
