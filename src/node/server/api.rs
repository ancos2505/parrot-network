mod client;

use h10::http::{request::Request, status_code::StatusCode};

use super::{result::ServerResult, ServerResponse};

use self::client::Client;

#[derive(Debug)]
pub(super) struct Api;

impl Api {
    pub(super) fn handler(request: &Request) -> ServerResult<ServerResponse> {
        let path_str = &**request.path();
        if path_str.starts_with("/api/client/") {
            Client::handler(request)
        } else {
            Ok(ServerResponse::new(StatusCode::NotFound))
        }
    }
}
