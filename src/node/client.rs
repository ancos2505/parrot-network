mod http_client;
pub(crate) mod result;

use std::{thread, time::Duration};

use h10::http::{
    headers::{Authorization, UserAgent, WWWAuthenticate},
    request::Request,
    response::Response,
    url_path::UrlPath,
};
use http_client::ParrotHttpClient;
use result::ClientError;

use crate::{
    proto::{
        blockchain::{
            result::{BlockchainProtoError, BlockchainProtoResult},
            wallet::PublicKey,
        },
        node_session::{
            fields::Realm,
            pki_client_challenge::{fields::ClientChallenge, PkiClientChallenge},
            pki_server_challenge::PkiServerChallenge,
        },
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
        let public_key = PublicKey::from(secret_key);

        let client_realm = Realm::ParrotNode;

        let pki_challenge = {
            let challenge = ClientChallenge::new();
            let signature = secret_key.sign(challenge.as_bytes());
            PkiClientChallenge::builder()
                .realm(client_realm.clone())
                .challenge(challenge)
                .signature(signature)
                .public_key(public_key)
                .finish()
        };

        let authorization = Authorization::new(&pki_challenge.to_string())?;

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

        let pki_server_challenge = parse_and_check_server_challenge(&response, &client_realm)?;

        println!(
            "NodeClient: (Server responded with realm [{:?}]",
            pki_server_challenge.realm()
        );

        Ok(())
    }
}

fn parse_and_check_server_challenge(
    response: &Response,
    client_realm: &Realm,
) -> BlockchainProtoResult<PkiServerChallenge> {
    use h10::http::headers::IntoHeader;
    let www_authenticate = response
        .headers()
        .get(WWWAuthenticate::default().into_header().name())
        .ok_or(BlockchainProtoError::PkiChallenge(
            "Invalid HTTP Response from server. Reason: There's no WWWAuthenticate header".into(),
        ))?;

    // * Check if server's challenge is valid - BEGIN

    let req_pki_server_challenge = www_authenticate.value().parse::<PkiServerChallenge>()?;

    let server_realm = req_pki_server_challenge.realm();
    let server_challenge = req_pki_server_challenge.challenge();
    let server_signature = req_pki_server_challenge.signature();
    let server_public_key = req_pki_server_challenge.public_key();

    // TODO: Implement more sophisticated validation
    if client_realm != server_realm || !server_challenge.verify(server_signature, server_public_key)
    {
        return Err(BlockchainProtoError::PkiChallenge(
            "Invalid Payload inside WWW-Authenticate header".into(),
        ));
    } else {

        // TODO: Add to an in-memory Database for servers in handshaking process
    }
    // * Check if client's server_challenge is valid  - END

    Ok(req_pki_server_challenge)
}
