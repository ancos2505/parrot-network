mod new;

use h10::http::{request::Request, status_code::StatusCode};
use new::NewPeer;

use crate::node::server::{result::ServerResult, ServerResponse};

#[derive(Debug)]
pub(super) struct Client;

impl Client {
    pub(super) fn handler(request: &Request) -> ServerResult<ServerResponse> {
        let path_str = &**request.path();
        if path_str == "/api/client/new" {
            NewPeer::handler(request)
        } else {
            Ok(ServerResponse::new(StatusCode::NotFound))
        }
    }
}
