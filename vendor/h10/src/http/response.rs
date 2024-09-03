pub mod parser;

use std::fmt::{Debug, Display};

use parser::ResponseParser;

use super::{
    body::Body,
    headers::{Headers, IntoHeader},
    result::H10LibResult,
    status_code::StatusCode,
    version::Version,
};

#[derive(Debug)]
pub struct Response {
    http_version: Version,
    status: StatusCode,
    headers: Headers,
    body: Option<Body>,
}

impl Response {
    // * Builders
    pub fn new(status: StatusCode) -> Self {
        Self {
            http_version: Version::Http1_0,
            status,
            headers: Headers::empty(),
            body: None,
        }
    }

    // TODO
    pub fn add_header<H: IntoHeader>(mut self, header: H) -> Self {
        let header_entry = header.into_header();
        self.headers.add(header_entry);
        Self {
            http_version: self.http_version,
            status: self.status,
            headers: self.headers,
            body: self.body,
        }
    }

    // TODO
    pub fn set_body<B: AsRef<str>>(self, body: B) -> Self {
        use crate::http::headers::ContentLength;
        let body = Body::new_unchecked(body.as_ref());
        let response = self.add_header(ContentLength::length(body.len() + 1));

        Self {
            http_version: response.http_version,
            status: response.status,
            headers: response.headers,
            body: Some(body.into()),
        }
    }

    // * Getters
    pub fn http_version(&self) -> &Version {
        &self.http_version
    }

    pub fn status(&self) -> &StatusCode {
        &self.status
    }

    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    pub fn body(&self) -> Option<&Body> {
        self.body.as_ref()
    }

    // * Public methods
    pub fn parse(bytes: &[u8]) -> H10LibResult<Self> {
        ResponseParser::parse(bytes)
    }
    pub fn into_inner(self) -> (Version, StatusCode, Headers, Option<Body>) {
        let Self {
            http_version,
            status,
            headers,
            body,
        } = self;
        (http_version, status, headers, body)
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "".to_owned();
        output.push_str(&self.http_version.to_string());
        output.push(' ');
        output.push_str(&self.status.to_string());
        output.push_str("\r\n");

        for header_entry in self.headers.iter() {
            output.push_str(&header_entry.to_string());
            output.push_str("\r\n");
        }

        output.push_str("\r\n");

        if let Some(body) = &self.body {
            output.push_str(&body.to_string());
            output.push_str("\n");
        }

        write!(f, "{}", output)
    }
}
