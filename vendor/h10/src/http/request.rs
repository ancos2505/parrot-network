pub mod builder;
pub mod parser;

#[cfg(test)]
mod tests;

use std::fmt::Display;

use builder::{
    RequestBuilderDelete, RequestBuilderGet, RequestBuilderHead, RequestBuilderLink,
    RequestBuilderPost, RequestBuilderPut, RequestBuilderUnlink,
};
use parser::RequestParser;

use super::{
    body::Body,
    headers::Headers,
    method::Method,
    query_string::{QsEntry, QueryString},
    result::H10LibResult,
    url_path::UrlPath,
    version::Version,
};

#[derive(Debug)]
pub struct Request {
    pub http_version: Version,
    pub method: Method,
    pub path: UrlPath,
    pub query_string: QueryString,
    pub headers: Headers,
    pub body: Option<Body>,
}

impl Request {
    // * Builders
    pub fn delete() -> RequestBuilderDelete {
        RequestBuilderDelete
    }
    pub fn get() -> RequestBuilderGet {
        RequestBuilderGet
    }
    pub fn head() -> RequestBuilderHead {
        RequestBuilderHead
    }
    pub fn link() -> RequestBuilderLink {
        RequestBuilderLink
    }
    pub fn post() -> RequestBuilderPost {
        RequestBuilderPost
    }
    pub fn put() -> RequestBuilderPut {
        RequestBuilderPut
    }
    pub fn unlink() -> RequestBuilderUnlink {
        RequestBuilderUnlink
    }

    // * Getters
    pub fn http_version(&self) -> &Version {
        &self.http_version
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn path(&self) -> &UrlPath {
        &self.path
    }

    pub fn query_string(&self) -> &QueryString {
        &self.query_string
    }

    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    pub fn body(&self) -> Option<&Body> {
        self.body.as_ref()
    }

    // * Public methods
    pub fn parse(bytes: &[u8]) -> H10LibResult<Self> {
        RequestParser::parse(bytes)
    }
}

impl Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "".to_owned();
        output.push_str(&self.method.to_string());
        output.push(' ');
        output.push_str(&self.path.to_string());
        output.push(' ');
        output.push_str(&self.http_version.to_string());
        output.push_str("\r\n");

        for header_entry in self.headers.iter() {
            output.push_str(&header_entry.to_string());
            output.push_str("\r\n");
        }
        output.push_str("\r\n");
        if let Some(body) = &self.body {
            output.push_str(&body.to_string());
        }

        write!(f, "{}", output)
    }
}
