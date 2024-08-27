mod error_404;
mod root;
mod styles_css;

use h10::http::request::Request;

use crate::{node::webui::CliVerboseMode, CLI_ARGS};

use super::WebuiResponse;

use self::styles_css::styles_css;

pub(crate) struct Endpoint;

impl Endpoint {
    pub(crate) fn dispatcher(raw_request: &[u8]) -> WebuiResponse {
        use super::pages::{error_404::error_404, root::root};

        let request = match Request::parse(raw_request) {
            Ok(req) => req,
            Err(err) => {
                if let Some(cli) = CLI_ARGS.get() {
                    if cli.verbose == CliVerboseMode::Enabled {
                        eprintln!("Error: {err}");
                    }
                }
                return WebuiResponse::new(err.into());
            }
        };

        let res = match &**request.path() {
            "/" => root(request),
            "/assets/styles.css" => styles_css(),
            _ => error_404(),
        };

        match res {
            Ok(response) => return response,
            Err(err) => WebuiResponse::new(err.into()),
        }
    }
}
