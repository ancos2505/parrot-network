use core::str;
use std::rc::Rc;

use crate::{
    constants::MAX_RESPONSE_LENGTH,
    http::{
        body::Body,
        headers::{ContentLength, IntoHeader},
        result::{H10LibError, H10LibResult},
        status_code::StatusCode,
    },
};

use crate::http::{headers::Headers, version::Version};

use super::Response;

#[derive(Debug)]
pub struct ResponseParser;

impl ResponseParser {
    pub fn parse(bytes: &[u8]) -> H10LibResult<Response> {
        let now = std::time::Instant::now();
        if bytes.len() > MAX_RESPONSE_LENGTH {
            return Err(H10LibError::ResponseParser(format!(
                "Response size is larger than expected. MAX: {} Bytes",
                MAX_RESPONSE_LENGTH
            )));
        }

        let headers_region = Self::get_header_region(bytes)?;

        let status_line_bytes = Self::get_status_line_bytes(headers_region)?;

        let http_version = Self::parse_http_version(status_line_bytes)?;

        let status = Self::parse_statuscode(status_line_bytes)?;

        println!(
            "ResponseParser: Status-Line security check in {} secs",
            now.elapsed().as_secs_f64()
        );

        let valid_bytes = {
            let mut found = 0;
            for (idx, c) in bytes.iter().enumerate() {
                if *c == b'\0' {
                    found = idx;
                    break;
                }
            }
            let (valid, _) = bytes.split_at(found);
            valid
        };

        let request_str = std::str::from_utf8(valid_bytes)?;

        let rc_request_str: Rc<str> = request_str.into();

        let mut iter_response_parts = rc_request_str.split("\r\n\r\n");
        let headers_region = iter_response_parts
            .next()
            .ok_or(H10LibError::ResponseParser(
                "Invalid HTTP Response on split headers and body".into(),
            ))?;
        let headers = Headers::parse(headers_region)?;

        let mut maybe_body: Option<Body> = None;

        if let Some(body_region) = iter_response_parts.next() {
            let body_inner: Body = body_region.parse()?;
            let body_length_number = body_inner.len();

            let maybe_content_length = headers.get(ContentLength::default().into_header().name());
            if let Some(content_length) = maybe_content_length {
                let content_length_number = content_length.value().parse()?;
                if body_length_number == content_length_number {
                    maybe_body = Some(body_inner);
                } else {
                    return Err(H10LibError::InvalidInputData(
                        format!("Invalid body by Content-Length header. content_length_number: {}, body.len(): {}",content_length_number,body_length_number),
                    ));
                }
            } else {
                if body_length_number > 0 {
                    return Err(H10LibError::InvalidInputData(
                        "Invalid body by nonexistence of Content-Length header".into(),
                    ));
                }
            }
        }

        Ok(Response {
            http_version,
            status,
            headers,
            body: maybe_body,
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
        Err(H10LibError::ResponseParser(
            "Invalid HTTP Response Header region".into(),
        ))
    }

    fn get_status_line_bytes<'a>(input: &'a [u8]) -> H10LibResult<&'a [u8]> {
        let seq = b"\r\n";
        let mut i = 0;
        while i + seq.len() <= input.len() {
            if input[i..i + seq.len()] == *seq {
                return Ok(&input[..i]);
            }
            i += 1;
        }
        Err(H10LibError::ResponseParser(
            "Invalid HTTP Response header line".into(),
        ))
    }

    fn parse_statuscode(input: &[u8]) -> H10LibResult<StatusCode> {
        use std::str;

        let mut iter = input.split(|b| *b == b' ');

        let _discard_version = iter.next();

        let bytes = iter.next().ok_or(H10LibError::ResponseParser(
            "Malformed bytes HTTP Method line on parsing Method".into(),
        ))?;

        if bytes.len() > StatusCode::MAX_LENGTH {
            return Err(H10LibError::ResponseParser(format!(
                "HTTP StatusCode payload size is larger than expected. MAX: {} Bytes. Found: {} Bytes",
                StatusCode::MAX_LENGTH,
                input.len()
            )));
        }

        let method_str = str::from_utf8(bytes)?;
        method_str.parse()
    }

    fn parse_http_version(input: &[u8]) -> H10LibResult<Version> {
        let bytes = input
            .split(|b| *b == b' ')
            .next()
            .ok_or(H10LibError::ResponseParser(
                "Malformed bytes HTTP Method lineon parsing HTTP Version".into(),
            ))?;

        if bytes.len() > Version::MAX_LENGTH {
            return Err(H10LibError::ResponseParser(format!(
                "HTTP Version payload size is larger than expected. MAX: {} Bytes. Found: {} Bytes",
                Version::MAX_LENGTH,
                input.len()
            )));
        }

        let version = std::str::from_utf8(bytes)?;
        version.parse()
    }
}
