use h10::http::{method::Method, request::Request, status_code::StatusCode};

use super::{result::ServerResult, ServerResponse};

#[derive(Debug)]
pub(super) struct HealthCheck;

impl HealthCheck {
    pub(super) fn handler(request: &Request) -> ServerResult<ServerResponse> {
        if *request.method() == Method::Get {
            Ok(ServerResponse::new(StatusCode::OK))
        } else {
            Ok(ServerResponse::new(StatusCode::NotFound))
        }
    }
}
