use crate::http::{
    headers::{HeaderEntry, HeaderName, HeaderValue, IntoHeader},
    result::H10LibResult,
};

/// ### Server header
/// Related: Server/2.14 Library/3.57
///
/// Reference: https://www.rfc-editor.org/rfc/rfc1945.html#section-10.14
#[derive(Debug, PartialEq, Eq)]
pub struct Server {
    name: HeaderName,
    value: HeaderValue,
}

impl Default for Server {
    fn default() -> Self {
        let value = format!(
            "{} (v{})",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        );

        Self {
            name: HeaderName::new_unchecked("Server"),
            value: HeaderValue::new_unchecked(&value),
        }
    }
}

impl Server {
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

impl IntoHeader for Server {
    fn into_header(self) -> HeaderEntry {
        let Self { name, value } = self;
        HeaderEntry { name, value }
    }
}
