mod error_404;
mod pico_min_css;
mod root;
mod styles_css;

use h10::http::request::Request;

use crate::NODE_CONFIG;

use self::pico_min_css::pico_min_css;
use self::styles_css::styles_css;

use super::WebUiResponse;

pub(crate) struct Endpoint;

impl Endpoint {
    pub(crate) fn dispatcher(raw_request: &[u8]) -> WebUiResponse {
        use super::pages::{error_404::error_404, root::root};

        let request = match Request::parse(raw_request) {
            Ok(req) => req,
            Err(err) => {
                if let Some(node_config) = NODE_CONFIG.get() {
                    if node_config.cli().verbose() {
                        eprintln!("Error: {err}");
                    }
                }
                return WebUiResponse::new(err.into());
            }
        };

        let res = match &**request.path() {
            "/" => root(request),
            "/assets/styles.css" => styles_css(),
            "/assets/pico.min.css" => pico_min_css(),
            _ => error_404(),
        };

        match res {
            Ok(response) => return response,
            Err(err) => WebUiResponse::new(err.into()),
        }
    }
}
