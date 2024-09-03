use h10::http::{request::Request, status_code::StatusCode};

use crate::NODE_CONFIG;

use super::{api::Api, health_check::HealthCheck, ServerResponse, ServerResult};

pub(crate) struct Endpoint;

impl Endpoint {
    pub(crate) fn dispatcher(raw_request: &[u8]) -> ServerResponse {
        let request = match Request::parse(raw_request) {
            Ok(req) => req,
            Err(err) => {
                if let Some(node_config) = NODE_CONFIG.get() {
                    if node_config.cli().verbose() {
                        eprintln!("Error: {err}");
                    }
                }
                return ServerResponse::new(err.into());
            }
        };

        let path_str = &**request.path();

        let res: ServerResult<ServerResponse> = if path_str == "/health" {
            HealthCheck::handler(&request)
        } else if path_str.starts_with("/api/") {
            Api::handler(&request)
        } else {
            Ok(ServerResponse::new(StatusCode::NotFound))
        };
        match res {
            Ok(response) => return response,
            Err(err) => ServerResponse::new(err.into()),
        }
    }
}
