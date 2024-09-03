mod aditional;
pub mod builder;
mod common;
mod compat;
mod entity;
mod request;
mod response;

use std::fmt::{Debug, Display};

use builder::HeaderEntryBuilder;

pub use self::{
    aditional::{
        Accept, AcceptCharset, AcceptEncoding, AcceptLanguage, ContentLanguage, Link, MIMEVersion,
        RetryAfter, Title, URI,
    },
    common::{Date, Pragma},
    compat::Connection,
    entity::{Allow, ContentEncoding, ContentLength, ContentType, Expires, LastModified},
    request::{Authorization, From, IfModifiedSince, Referer, UserAgent},
    response::{Location, Server, WWWAuthenticate},
};

pub trait IntoHeader {
    fn into_header(self) -> HeaderEntry;
}

use std::{ops::Deref, rc::Rc, str::FromStr};

use crate::http::result::{H10LibError, H10LibResult};

#[derive(Debug, PartialEq, Eq)]
pub struct Headers(Vec<HeaderEntry>);
impl Headers {
    pub fn empty() -> Self {
        Self(vec![])
    }
    pub fn default_for_request() -> Self {
        let mut headers = Self::empty();

        headers.add(UserAgent::default().into_header());

        headers
    }
    pub fn add(&mut self, header_entry: HeaderEntry) {
        let mut maybe_idx: Option<usize> = None;
        for (idx, header) in self.0.iter().enumerate() {
            if header.name() == header_entry.name() {
                maybe_idx = Some(idx);
                break;
            } else {
                continue;
            }
        }

        if let Some(found_idx) = maybe_idx {
            self.0[found_idx] = header_entry;
        } else {
            self.0.push(header_entry);
        }
    }
    pub fn get(&self, header_name: &HeaderName) -> Option<&HeaderEntry> {
        let mut maybe_idx: Option<usize> = None;
        for (idx, header) in self.0.iter().enumerate() {
            if header.name() == header_name {
                maybe_idx = Some(idx);
                break;
            } else {
                continue;
            }
        }
        maybe_idx.and_then(|idx| self.0.get(idx))
    }

    pub fn parse(s: &str) -> H10LibResult<Self> {
        let headers_str = if s.len() > 0 {
            s
        } else {
            return Ok(Headers::empty());
        };

        let mut headers = vec![];

        let mut iter = headers_str.split("\r\n");

        let _discard_status_line = iter.next();

        while let Some(entry) = iter.next() {
            headers.push(entry.parse()?);
        }
        Ok(Self(headers))
    }
}

impl Deref for Headers {
    type Target = Vec<HeaderEntry>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct HeaderEntry {
    name: HeaderName,
    value: HeaderValue,
}

impl HeaderEntry {
    pub fn builder() -> HeaderEntryBuilder {
        HeaderEntryBuilder
    }

    pub fn into_inner(self) -> (HeaderName, HeaderValue) {
        let Self { name, value } = self;
        (name, value)
    }

    pub fn name(&self) -> &HeaderName {
        &self.name
    }

    pub fn value(&self) -> &HeaderValue {
        &self.value
    }
}

impl FromStr for HeaderEntry {
    type Err = H10LibError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if let Some((key, value)) = input.split_once(":") {
            Ok(Self {
                name: key.trim().parse()?,
                value: value.trim().parse()?,
            })
        } else {
            Err(H10LibError::HeadersParser(
                "Malformed HTTP Header entry".into(),
            ))
        }
    }
}

impl Display for HeaderEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct HeaderName(Rc<str>);

impl HeaderName {
    pub fn new_unchecked<S: AsRef<str>>(name: S) -> Self {
        let s = name.as_ref();
        Self(s.into())
    }
}

impl Deref for HeaderName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for HeaderName {
    type Err = H10LibError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.into()))
    }
}

impl Display for HeaderName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct HeaderValue(Rc<str>);

impl HeaderValue {
    pub fn new_unchecked<S: AsRef<str>>(name: S) -> Self {
        let s = name.as_ref();
        Self(s.into())
    }
}

impl Deref for HeaderValue {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for HeaderValue {
    type Err = H10LibError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.into()))
    }
}

impl Display for HeaderValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
