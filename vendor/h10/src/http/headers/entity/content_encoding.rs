use crate::http::headers::{HeaderName, HeaderValue};

/// ### Content-Encoding
/// Related: Compression, Encryption, Base64
///
///  When present, its value indicates what additional content coding has been
/// applied to the resource, and thus what decoding mechanism must be applied in
/// order to obtain the media-type referenced by the Content-Type header field.
///

///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#section-10.3
#[derive(Debug, PartialEq, Eq)]
pub struct ContentEncoding {
    name: HeaderName,
    value: HeaderValue,
}
impl Default for ContentEncoding {
    fn default() -> Self {
        Self {
            name: HeaderName::new_unchecked("Content-Encoding"),
            value: HeaderValue::new_unchecked("Not_Defined"),
        }
    }
}

impl ContentEncoding {
    pub fn gzip() -> Self {
        Self {
            value: HeaderValue::new_unchecked("x-gzip"),
            ..Default::default()
        }
    }
}
