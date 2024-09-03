use std::marker::PhantomData;

use crate::http::{
    headers::{ContentLength, IntoHeader},
    method::Method,
    url_path::UrlPath,
    version::Version,
};

use super::{Body, Headers, QsEntry, QueryString, Request};

pub trait RequestBuilderState {}

#[derive(Debug)]
pub struct WithBody;

#[derive(Debug)]
pub struct WithOutBody;

impl RequestBuilderState for WithBody {}

impl RequestBuilderState for WithOutBody {}

#[derive(Debug)]
pub struct RequestBuilderDelete;

impl RequestBuilderDelete {
    pub fn path(self, path: UrlPath) -> Step3<WithOutBody> {
        let http_version = Version::Http1_0;
        let method = Method::Delete;

        Step3 {
            http_version,
            method,
            path,
            query_string: QueryString::empty(),
            headers: Headers::default_for_request(),
            _state: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct RequestBuilderGet;
impl RequestBuilderGet {
    pub fn path(self, path: UrlPath) -> Step3<WithOutBody> {
        let http_version = Version::Http1_0;
        let method = Method::Get;

        Step3 {
            http_version,
            method,
            path,
            query_string: QueryString::empty(),
            headers: Headers::default_for_request(),
            _state: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct RequestBuilderHead;

impl RequestBuilderHead {
    pub fn path(self, path: UrlPath) -> Step3<WithOutBody> {
        let http_version = Version::Http1_0;
        let method = Method::Head;

        Step3 {
            http_version,
            method,
            path,
            query_string: QueryString::empty(),
            headers: Headers::default_for_request(),
            _state: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct RequestBuilderLink;

impl RequestBuilderLink {
    pub fn path(self, path: UrlPath) -> Step3<WithOutBody> {
        let http_version = Version::Http1_0;
        let method = Method::Link;

        Step3 {
            http_version,
            method,
            path,
            query_string: QueryString::empty(),
            headers: Headers::default_for_request(),
            _state: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct RequestBuilderPost;

impl RequestBuilderPost {
    pub fn path(self, path: UrlPath) -> Step3<WithBody> {
        let http_version = Version::Http1_0;
        let method = Method::Post;

        Step3 {
            http_version,
            method,
            path,
            query_string: QueryString::empty(),
            headers: Headers::default_for_request(),
            _state: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct RequestBuilderPut;

impl RequestBuilderPut {
    pub fn path(self, path: UrlPath) -> Step3<WithBody> {
        let http_version = Version::Http1_0;
        let method = Method::Put;

        Step3 {
            http_version,
            method,
            path,
            query_string: QueryString::empty(),
            headers: Headers::default_for_request(),
            _state: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct RequestBuilderUnlink;

impl RequestBuilderUnlink {
    pub fn path(self, path: UrlPath) -> Step3<WithOutBody> {
        let http_version = Version::Http1_0;
        let method = Method::Unlink;

        Step3 {
            http_version,
            method,
            path,
            query_string: QueryString::empty(),
            headers: Headers::default_for_request(),
            _state: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct Step3<B: RequestBuilderState> {
    http_version: Version,
    method: Method,
    path: UrlPath,
    query_string: QueryString,
    headers: Headers,
    _state: PhantomData<B>,
}

impl Step3<WithOutBody> {
    pub fn add_qs_entry(self, qs_entry: QsEntry) -> Step3<WithOutBody> {
        let Self {
            http_version,
            method,
            path,
            mut query_string,
            headers,
            ..
        } = self;

        query_string.add(qs_entry);

        Step3 {
            http_version,
            method,
            path,
            query_string,
            headers,
            _state: PhantomData,
        }
    }
    pub fn add_header<H: IntoHeader>(self, header: H) -> Step3<WithOutBody> {
        let Self {
            http_version,
            method,
            path,
            query_string,
            mut headers,
            ..
        } = self;

        headers.add(header.into_header());

        Step3 {
            http_version,
            method,
            path,
            query_string,
            headers,
            _state: PhantomData,
        }
    }

    pub fn finish(self) -> Request {
        let Self {
            http_version,
            method,
            path,
            query_string,
            headers,
            ..
        } = self;

        Request {
            http_version,
            method,
            path,
            query_string,
            headers,
            body: None,
        }
    }
}

impl Step3<WithBody> {
    pub fn add_qs_entry(self, qs_entry: QsEntry) -> Step3<WithBody> {
        let Self {
            http_version,
            method,
            path,
            mut query_string,
            headers,
            ..
        } = self;

        query_string.add(qs_entry);

        Step3 {
            http_version,
            method,
            path,
            query_string,
            headers,
            _state: PhantomData,
        }
    }
    pub fn add_header<H: IntoHeader>(self, header: H) -> Step3<WithBody> {
        let Self {
            http_version,
            method,
            path,
            query_string,
            mut headers,
            ..
        } = self;

        headers.add(header.into_header());

        Step3 {
            http_version,
            method,
            path,
            query_string,
            headers,
            _state: PhantomData,
        }
    }
    pub fn set_body(self, body: Body) -> Step4 {
        let Self {
            http_version,
            method,
            path,
            query_string,
            mut headers,
            ..
        } = self;

        headers.add(ContentLength::length((*body).len()).into_header());

        Step4 {
            http_version,
            method,
            path,
            query_string,
            headers,
            body: Some(body),
        }
    }
}

#[derive(Debug)]
pub struct Step4 {
    http_version: Version,
    method: Method,
    path: UrlPath,
    query_string: QueryString,
    headers: Headers,
    body: Option<Body>,
}
impl Step4 {
    pub fn finish(self) -> Request {
        let Self {
            http_version,
            method,
            path,
            query_string,
            headers,
            body,
        } = self;

        Request {
            http_version,
            method,
            path,
            query_string,
            headers,
            body,
        }
    }
}
