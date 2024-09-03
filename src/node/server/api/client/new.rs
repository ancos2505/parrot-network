use ed25519_dalek::SigningKey;
use h10::http::{
    headers::{From, IntoHeader, UserAgent},
    method::Method,
    request::Request,
    status_code::StatusCode,
};

use crate::{
    node::server::{
        result::{ServerError, ServerResult},
        ServerResponse,
    },
    proto::helpers::hex_to_string::hex_array_32,
    NODE_CONFIG,
};

#[derive(Debug)]
pub(super) struct NewPeer;

impl NewPeer {
    pub(super) fn handler(request: &Request) -> ServerResult<ServerResponse> {
        if *request.method() == Method::Get {
            let maybe_user_agent = request
                .headers()
                .get(UserAgent::default().into_header().name());
            let maybe_user_from = request.headers().get(From::default().into_header().name());
            if let Some(user_agent) = maybe_user_agent {
                println!("NodeServer: (NewPeer): [{}]", user_agent);
            }

            if let Some(from) = maybe_user_from {
                println!("NodeServer: (NewPeer): [{}]", from);
            }
            let signing_key = NODE_CONFIG
                .get()
                .and_then(|config| {
                    config
                        .secret_key()
                        .map(|secret| SigningKey::from_bytes(&secret))
                })
                .ok_or(ServerError::NodeSigningKey(
                    "Error on getting signingkey".into(),
                ))?;

            let pubkey = signing_key.verifying_key();

            let pubkey_str = hex_array_32(&pubkey.to_bytes());

            let from = From::new(&pubkey_str)?;

            Ok(ServerResponse::new(StatusCode::OK).add_header(from))
        } else {
            Ok(ServerResponse::new(StatusCode::NotFound))
        }
    }
}
