use crate::http::headers::{HeaderEntry, HeaderName, HeaderValue, IntoHeader};

/// ### Connection
/// Related: HTTP/1.1 compatibility
///
///  "Persistent connections are the default for HTTP/1.1 messages; we introduce
/// a new keyword (Connection: close) for declaring non-persistence." -
/// **RFC2068#section-19.7.1**
///
/// References:
/// - https://www.rfc-editor.org/rfc/rfc2068#section-14.10
/// - https://www.rfc-editor.org/rfc/rfc2068#section-19.7.1
///
#[derive(Debug, PartialEq, Eq)]
pub struct Connection {
    name: HeaderName,
    value: HeaderValue,
}

impl Default for Connection {
    fn default() -> Self {
        Self {
            name: HeaderName::new_unchecked("Connection"),
            value: HeaderValue::new_unchecked("close"),
        }
    }
}

impl IntoHeader for Connection {
    fn into_header(self) -> HeaderEntry {
        let Self { name, value } = self;
        HeaderEntry { name, value }
    }
}
