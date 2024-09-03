use crate::http::headers::{HeaderEntry, HeaderName, HeaderValue, IntoHeader};

/// ### Retry-After
/// Related: Content handling
///
///  The Retry-After response-header field can be used with a 503 (service
/// unavailable) response to indicate how long the service is expected to be
/// unavailable to the requesting client. The value of this field can be either
/// an HTTP-date or an integer number of seconds (in decimal) after the time of
/// the response.
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#appendix-D.2.8
///
#[derive(Debug, PartialEq, Eq)]
pub struct RetryAfter {
    name: HeaderName,
    value: HeaderValue,
}

impl Default for RetryAfter {
    fn default() -> Self {
        Self {
            name: HeaderName::new_unchecked("Retry-After"),
            value: HeaderValue::new_unchecked("Not_Defined"),
        }
    }
}

impl IntoHeader for RetryAfter {
    fn into_header(self) -> HeaderEntry {
        let Self { name, value } = self;
        HeaderEntry { name, value }
    }
}
