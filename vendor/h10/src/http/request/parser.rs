use std::rc::Rc;

use crate::{
    constants::{AsciiWhiteSpace, MAX_REQUEST_LENGTH},
    http::result::{H10LibError, H10LibResult},
};

use crate::http::{
    headers::Headers, method::Method, query_string::QueryString, url_path::UrlPath,
    version::Version,
};

use super::Request;

#[derive(Debug)]
pub struct RequestParser;

impl RequestParser {
    pub fn parse(bytes: &[u8]) -> H10LibResult<Request> {
        let now = std::time::Instant::now();
        if bytes.len() > MAX_REQUEST_LENGTH {
            return Err(H10LibError::RequestParser(format!(
                "Request size is larger than expected. MAX: {} Bytes",
                MAX_REQUEST_LENGTH
            )));
        }

        let headers_region = Self::get_header_region(bytes)?;

        let request_line_bytes = Self::get_method_line_bytes(headers_region)?;

        let method = Self::parse_method(request_line_bytes)?;

        let http_version = Self::parse_http_version(request_line_bytes)?;

        println!(
            "Request-Line security check in {} secs",
            now.elapsed().as_secs_f64()
        );

        let request_str = std::str::from_utf8(bytes)?;

        let rc_request_str: Rc<str> = request_str.into();

        let (headers_region, body_region) =
            rc_request_str
                .split_once("\r\n\r\n")
                .ok_or(H10LibError::RequestParser(
                    "Invalid HTTP Request on split headers and body".into(),
                ))?;

        let first_line = headers_region
            .split("\r\n")
            .next()
            .ok_or(H10LibError::RequestParser(
                "Malformed HTTP Request Headers".into(),
            ))?;

        let (_, first_line_remaining) =
            first_line
                .split_once(AsciiWhiteSpace::as_str())
                .ok_or(H10LibError::RequestParser(
                    "Malformed HTTP Header Method line".into(),
                ))?;

        let (url_str, _) = first_line_remaining
            .split_once(AsciiWhiteSpace::as_str())
            .ok_or(H10LibError::RequestParser(
                "Malformed HTTP Header Method line on searching for Url path".into(),
            ))?;

        let (maybe_path_str, maybe_qs_str) = Self::parse_url(url_str)?;

        let path = match maybe_path_str {
            Some(inner_str) => UrlPath::parse(inner_str)?,
            None => UrlPath::root(),
        };

        let query_string = QueryString::parse(maybe_qs_str)?;

        let headers = Headers::parse(headers_region)?;

        let body = Some(body_region.parse()?);

        Ok(Request {
            method,
            http_version,
            path,
            query_string,
            headers,
            body,
        })
    }

    fn get_header_region<'a>(raw_request: &'a [u8]) -> H10LibResult<&'a [u8]> {
        let seq = b"\r\n\r\n";
        let mut i = 0;
        while i + seq.len() <= raw_request.len() {
            if raw_request[i..i + seq.len()] == *seq {
                return Ok(&raw_request[..(i + 2)]);
            }
            i += 1;
        }
        Err(H10LibError::RequestParser(
            "Invalid HTTP Request Header region".into(),
        ))
    }

    fn get_method_line_bytes<'a>(input: &'a [u8]) -> H10LibResult<&'a [u8]> {
        let seq = b"\r\n";
        let mut i = 0;
        while i + seq.len() <= input.len() {
            if input[i..i + seq.len()] == *seq {
                return Ok(&input[..i]);
            }
            i += 1;
        }
        Err(H10LibError::RequestParser(
            "Invalid HTTP Request header line".into(),
        ))
    }

    fn parse_method(input: &[u8]) -> H10LibResult<Method> {
        let bytes = input
            .split(|b| *b == b' ')
            .next()
            .ok_or(H10LibError::RequestParser(
                "Malformed bytes HTTP Method line on parsing Method".into(),
            ))?;

        if bytes.len() > Method::MAX_LENGTH {
            return Err(H10LibError::RequestParser(format!(
                "HTTP Method payload size is larger than expected. MAX: {} Bytes. Found: {} Bytes",
                Method::MAX_LENGTH,
                input.len()
            )));
        }

        let method_str = std::str::from_utf8(bytes)?;
        method_str.parse()
    }

    fn parse_http_version(input: &[u8]) -> H10LibResult<Version> {
        let mut iter = input.split(|b| *b == b' ');
        iter.next();
        iter.next();

        let bytes = iter.next().ok_or(H10LibError::RequestParser(
            "Malformed bytes HTTP Method lineon parsing HTTP Version".into(),
        ))?;

        if bytes.len() > Version::MAX_LENGTH {
            return Err(H10LibError::RequestParser(format!(
                "HTTP Version payload size is larger than expected. MAX: {} Bytes. Found: {} Bytes",
                Version::MAX_LENGTH,
                input.len()
            )));
        }

        let version = std::str::from_utf8(bytes)?;
        version.parse()
    }

    fn parse_url<'a>(input: &'a str) -> H10LibResult<(Option<&'a str>, Option<&'a str>)> {
        let trimmed = input.trim();
        if trimmed.contains("?") {
            let (path_str, query_string) = input.split_once("?").ok_or(
                H10LibError::RequestParser("Malformed UrlPath in HTTP Header Method line".into()),
            )?;
            Ok((Some(path_str), Some(query_string)))
        } else {
            Ok((Some(trimmed), None))
        }
    }
}
