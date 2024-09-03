use crate::http::headers::{HeaderEntry, HeaderName, HeaderValue, IntoHeader};

/// ### Accept-Charset
/// Related: Content handling
///
///  The Accept-Charset request-header field can be used to indicate a list of
/// preferred character sets other than the default US-ASCII and ISO-8859-1.
/// This field allows clients capable of understanding more comprehensive or
/// special-purpose character sets to signal that capability to a server which
/// is capable of representing documents in those character sets.
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#appendix-D.2.2
///
#[derive(Debug, PartialEq, Eq)]
pub struct AcceptCharset {
    name: HeaderName,
    value: HeaderValue,
}

impl Default for AcceptCharset {
    fn default() -> Self {
        Self {
            name: HeaderName::new_unchecked("Accept-Charset"),
            value: HeaderValue::new_unchecked("Not_Defined"),
        }
    }
}

impl IntoHeader for AcceptCharset {
    fn into_header(self) -> HeaderEntry {
        let Self { name, value } = self;
        HeaderEntry { name, value }
    }
}
