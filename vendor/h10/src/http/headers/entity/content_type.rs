use crate::http::headers::{HeaderEntry, HeaderName, HeaderValue, IntoHeader};

/// ### Content-Type header
/// Related: Entity-Body
///
///  The Content-Type entity-header field indicates the media type of the
/// Entity-Body sent to the recipient or, in the case of the HEAD method, the
/// media type that would have been sent had the request been a GET.
///
///  If the media type remains unknown, the recipient should treat it as type
/// "application/octet-stream".
///
/// **Reference:** https://www.rfc-editor.org/rfc/rfc1945.html#section-10.5
///
#[derive(Debug, PartialEq, Eq)]
pub struct ContentType {
    name: HeaderName,
    value: HeaderValue,
}

impl Default for ContentType {
    fn default() -> Self {
        Self {
            name: HeaderName::new_unchecked("Content-Type"),
            value: HeaderValue::new_unchecked("Not_Defined"),
        }
    }
}
impl ContentType {
    pub fn octet_stream() -> Self {
        Self {
            value: HeaderValue::new_unchecked("application/octet-stream"),
            ..Default::default()
        }
    }
    pub fn html() -> Self {
        Self {
            value: HeaderValue::new_unchecked("text/html; charset=UTF-8"),
            ..Default::default()
        }
    }

    pub fn css() -> Self {
        Self {
            value: HeaderValue::new_unchecked("text/css; charset=UTF-8"),
            ..Default::default()
        }
    }

    pub fn javascript() -> Self {
        Self {
            value: HeaderValue::new_unchecked("application/javascript; charset=UTF-8"),
            ..Default::default()
        }
    }
    pub fn json() -> Self {
        Self {
            value: HeaderValue::new_unchecked("application/json; charset=UTF-8"),
            ..Default::default()
        }
    }
    pub fn form_url_encoded() -> Self {
        Self {
            value: HeaderValue::new_unchecked("application/x-www-form-urlencoded"),
            ..Default::default()
        }
    }
}

impl IntoHeader for ContentType {
    fn into_header(self) -> HeaderEntry {
        let Self { name, value } = self;
        HeaderEntry { name, value }
    }
}
