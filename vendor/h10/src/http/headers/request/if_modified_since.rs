use crate::http::headers::{HeaderEntry, HeaderName, HeaderValue, IntoHeader};

/// ### If-Modified-Since
/// Related: Resource state
/// 304 (not modified) response will be returned without any
///          Entity-Body.
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#section-10.9
///
#[derive(Debug, PartialEq, Eq)]
pub struct IfModifiedSince {
    name: HeaderName,
    value: HeaderValue,
}

impl Default for IfModifiedSince {
    fn default() -> Self {
        Self {
            name: HeaderName::new_unchecked("If-Modified-Since"),
            value: HeaderValue::new_unchecked("Not_Defined"),
        }
    }
}

impl IntoHeader for IfModifiedSince {
    fn into_header(self) -> HeaderEntry {
        let Self { name, value } = self;
        HeaderEntry { name, value }
    }
}
