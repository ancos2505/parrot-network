use std::{ops::Deref, rc::Rc};

use crate::{
    constants::URL_PARTS_MAX_CHARS,
    http::result::{H10LibError, H10LibResult},
};

#[derive(Debug, PartialEq, Eq)]
pub struct UrlPath(Rc<str>);

impl UrlPath {
    pub fn new_unchecked(s: &str) -> Self {
        Self(s.into())
    }

    pub fn root() -> Self {
        Self::new_unchecked("/")
    }
}

impl Deref for UrlPath {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl UrlPath {
    pub fn parse<S: AsRef<str>>(s: S) -> H10LibResult<Self> {
        let input = s.as_ref();
        if input.chars().count() <= URL_PARTS_MAX_CHARS {
            Ok(Self(input.into()))
        } else {
            Err(H10LibError::InvalidInputData(format!(
                "UrlPath length is larger than expected. MAX {}",
                URL_PARTS_MAX_CHARS
            )))
        }
    }
}
