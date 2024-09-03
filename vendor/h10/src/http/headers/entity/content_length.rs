use crate::http::headers::{HeaderEntry, HeaderName, HeaderValue, IntoHeader};

/// ### Content-Length header
/// Related: Entity-Body
///
///  The Content-Length entity-header field indicates the size of the
/// Entity-Body, in decimal number of octets, sent to the recipient or, in the
/// case of the HEAD method, the size of the Entity-Body that would have been
/// sent had the request been a GET.
///
/// **Reference:** https://www.rfc-editor.org/rfc/rfc1945.html#section-10.4
///
#[derive(Debug, PartialEq, Eq)]
pub struct ContentLength {
    name: HeaderName,
    value: HeaderValue,
}
impl Default for ContentLength {
    fn default() -> Self {
        Self {
            name: HeaderName::new_unchecked("Content-Length"),
            value: HeaderValue::new_unchecked("Not_Defined"),
        }
    }
}
impl ContentLength {
    pub fn length(len: usize) -> Self {
        Self {
            value: HeaderValue::new_unchecked(len.to_string()),
            ..Default::default()
        }
    }
}
impl IntoHeader for ContentLength {
    fn into_header(self) -> HeaderEntry {
        let Self { name, value } = self;
        HeaderEntry { name, value }
    }
}
