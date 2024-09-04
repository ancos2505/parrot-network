mod http_client;
pub(crate) mod result;

use std::{thread, time::Duration};

use h10::http::{
    headers::{Authorization, UserAgent},
    request::Request,
    response::parser::ResponseParser,
    url_path::UrlPath,
};
use http_client::ParrotHttpClient;
use result::ClientError;

use crate::{
    proto::{
        blockchain::wallet::{PublicKey, SecretKey},
        helpers::hex_to_string::hex_slice,
    },
    NODE_CONFIG,
};

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

        let secret_key = NODE_CONFIG
            .get()
            .and_then(|config| config.secret_key())
            .ok_or(ClientError::NodeSigningKey(
                "Error on getting signingkey".into(),
            ))?;

        let pubkey = PublicKey::from(secret_key);

        let challenge: [u8; 8] = {
            use rand::{rngs::OsRng, RngCore};
            let mut inner_buf = [0u8; 8];
            OsRng.fill_bytes(&mut inner_buf);
            inner_buf
        };

        let signature = secret_key.sign(&challenge);

        let challenge_str = hex_slice(&challenge);

        let realm_str = format!("{pubkey}");

        let header_value = format!(
            r#"PKI realm="{realm_str}", challenge="{challenge_str}", signature="{signature}", public_key="{pubkey}""#,
        );

        let authorization = Authorization::new(&header_value)?;

        let user_agent = UserAgent::custom(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))?;

        let request = Request::get()
            .path(UrlPath::new_unchecked("/api/client/new"))
            .add_header(user_agent)
            .add_header(authorization)
            .finish();

        println!("NodeServer: Send request to {}", peer);

        println!("NodeClient: Sending Request...");
        println!("{}", request);
        let timeout = Duration::from_secs(5);

        let response = match ParrotHttpClient::launch(request, peer.to_string(), timeout) {
            Ok(res) => res,
            Err(err) => {
                // TODO Blocklist peer;
                // dbg!(&err);
                return Err(err.into());
            }
        };

        let elapsed = start.elapsed().as_secs_f32();
        println!("NodeClient: Response received:\n{response}");
        println!("NodeClient: StatusCode: {}", response.status());
        println!("NodeClient: Response received in {} secs", elapsed);

        Ok(())
    }
}
