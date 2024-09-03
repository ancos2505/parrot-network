use crate::http::{
    headers::{HeaderEntry, HeaderName, HeaderValue, IntoHeader},
    result::H10LibResult,
};

/// ### User-Agent
/// Related: Browser/2.14 Library/3.57
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#section-10.15
#[derive(Debug, PartialEq, Eq)]
pub struct UserAgent {
    name: HeaderName,
    value: HeaderValue,
}

impl Default for UserAgent {
    fn default() -> Self {
        Self {
            name: HeaderName::new_unchecked("User-Agent"),
            value: HeaderValue::new_unchecked(library_str().as_str()),
        }
    }
}

impl UserAgent {
    pub fn custom(product_name: &str, product_version: &str) -> H10LibResult<Self> {
        let new_value = format!(
            "{prodname}/{prodversion} {library}",
            prodname = product_name,
            prodversion = product_version,
            library = library_str()
        );

        Ok(Self {
            value: new_value.parse()?,
            ..Default::default()
        })
    }
}

fn library_str() -> String {
    format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}

impl IntoHeader for UserAgent {
    fn into_header(self) -> HeaderEntry {
        let Self { name, value } = self;
        HeaderEntry { name, value }
    }
}
