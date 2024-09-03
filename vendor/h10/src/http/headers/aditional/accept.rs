use crate::http::headers::{HeaderEntry, HeaderName, HeaderValue, IntoHeader};

/// ### Accept
/// Related: Content handling
///
///  The Accept request-header field can be used to indicate a list of media
/// ranges which are acceptable as a response to the request. The asterisk "*"
/// character is used to group media types into ranges, with "*/*" indicating
/// all media types and "type/*" indicating all subtypes of that type. The set
/// of ranges given by the client should represent what types are acceptable
/// given the context of the request.
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#appendix-D.2.1
///
#[derive(Debug, PartialEq, Eq)]
pub struct Accept {
    name: HeaderName,
    value: HeaderValue,
}

impl Default for Accept {
    fn default() -> Self {
        Self {
            name: HeaderName::new_unchecked("Accept"),
            value: HeaderValue::new_unchecked("Not_Defined"),
        }
    }
}

impl IntoHeader for Accept {
    fn into_header(self) -> HeaderEntry {
        let Self { name, value } = self;
        HeaderEntry { name, value }
    }
}
