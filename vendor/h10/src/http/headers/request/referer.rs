use crate::http::headers::{HeaderEntry, HeaderName, HeaderValue, IntoHeader};

/// ### Referer
/// Related:  back-links to resources for interest, logging, optimized caching,
///          etc.
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#section-10.13
#[derive(Debug, PartialEq, Eq)]
pub struct Referer {
    name: HeaderName,
    value: HeaderValue,
}

impl Default for Referer {
    fn default() -> Self {
        Self {
            name: HeaderName::new_unchecked("Referer"),
            value: HeaderValue::new_unchecked("Not_Defined"),
        }
    }
}

impl IntoHeader for Referer {
    fn into_header(self) -> HeaderEntry {
        let Self { name, value } = self;
        HeaderEntry { name, value }
    }
}
