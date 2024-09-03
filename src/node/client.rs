mod http_client;
pub(crate) mod result;

use std::{thread, time::Duration};

use ed25519_dalek::SigningKey;
use h10::http::{
    headers::{From, UserAgent},
    request::Request,
    response::parser::ResponseParser,
    url_path::UrlPath,
};
use http_client::ParrotHttpClient;
use result::ClientError;

use crate::{proto::helpers::hex_to_string::hex_array_32, NODE_CONFIG};

use self::result::ClientResult;

use super::PeerConfig;

pub(crate) struct NodeClient;

impl NodeClient {
    pub(crate) fn run() -> ClientResult<()> {
        if let Some(node_config) = NODE_CONFIG.get() {
            for peer in node_config.toml().peers() {
                let th_result =
                    thread::spawn(|| -> ClientResult<()> { Self::request(peer) }).join();
                match th_result {
                    Ok(inner_res) => {
                        if let Err(client_error) = inner_res {
                            println!("NodeClient: Client error: {client_error:?}");
                        }
                    }
                    Err(err) => {
                        println!("NodeClient: Internal error: {err:?}");
                    }
                }
            }
        }

        Ok(())
    }

    // TODO: Implement Hostname/IP validation
    fn request(peer: &PeerConfig) -> ClientResult<()> {
        use std::time::Instant;

        let start = Instant::now();

        let signing_key = NODE_CONFIG
            .get()
            .and_then(|config| {
                config
                    .secret_key()
                    .map(|secret| SigningKey::from_bytes(&secret))
            })
            .ok_or(ClientError::NodeSigningKey(
                "Error on getting signingkey".into(),
            ))?;

        let pubkey = signing_key.verifying_key();

        let pubkey_str = hex_array_32(&pubkey.to_bytes());

        let header_from = From::new(&pubkey_str)?;

        let user_agent = UserAgent::custom(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))?;

        let request = Request::get()
            .path(UrlPath::new_unchecked("/api/client/new"))
            .add_header(user_agent)
            .add_header(header_from)
            .finish();

        println!("NodeServer: Send request to {}", peer);

        println!("NodeClient: Sending Request:\n{request}");

        let timeout = Duration::from_secs(5);

        let response_str = match ParrotHttpClient::launch(request, peer.to_string(), timeout) {
            Ok(res) => res,
            Err(err) => {
                // TODO Blocklist peer;
                // dbg!(&err);
                return Err(err.into());
            }
        };

        let response = ResponseParser::parse(response_str.as_bytes())?;

        let elapsed = start.elapsed().as_secs_f32();
        println!("NodeClient: Response received:\n{response}");
        println!("NodeClient: StatusCode: {}", response.status());
        println!("NodeClient: Response received in {} secs", elapsed);

        Ok(())
    }
}
