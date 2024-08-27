mod error_404;
mod root;
mod styles_css;

use h10::http::request::Request;

use crate::{
    node::webui::{
        // CliHttp10StrictMode,
        CliVerboseMode,
        ServerResponse,
    },
    CLI_ARGS,
};

use self::styles_css::styles_css;

pub struct Endpoint;

impl Endpoint {
    pub fn dispatcher(raw_request: &[u8]) -> ServerResponse {
        use super::pages::{error_404::error_404, root::root};

        let request = match Request::parse(raw_request) {
            Ok(req) => req,
            Err(err) => {
                if let Some(cli) = CLI_ARGS.get() {
                    if cli.verbose == CliVerboseMode::Enabled {
                        eprintln!("Error: {err}");
                    }
                }
                return ServerResponse::new(err.into());
            }
        };

        let res = match &**request.path() {
            "/" => root(request),
            "/assets/styles.css" => styles_css(),
            _ => error_404(),
        };

        match res {
            Ok(response) => return response,
            Err(err) => ServerResponse::new(err.into()),
        }
    }
}
