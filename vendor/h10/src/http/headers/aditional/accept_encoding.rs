use crate::http::headers::{HeaderEntry, HeaderName, HeaderValue, IntoHeader};

/// ### Accept-Encoding
/// Related: Content handling
///
///  The Accept-Encoding request-header field is similar to Accept, but
/// restricts the content-coding values which are acceptable in the response.
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#appendix-D.2.3
///
#[derive(Debug, PartialEq, Eq)]
pub struct AcceptEncoding {
    name: HeaderName,
    value: HeaderValue,
}

impl Default for AcceptEncoding {
    fn default() -> Self {
        Self {
            name: HeaderName::new_unchecked("Accept-Encoding"),
            value: HeaderValue::new_unchecked("Not_Defined"),
        }
    }
}

impl IntoHeader for AcceptEncoding {
    fn into_header(self) -> HeaderEntry {
        let Self { name, value } = self;
        HeaderEntry { name, value }
    }
}
