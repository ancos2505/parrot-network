use h10::http::{
    headers::{Authorization, From, IntoHeader, UserAgent, WWWAuthenticate},
    method::Method,
    request::Request,
    status_code::StatusCode,
};

use crate::{
    node::server::{
        result::{ServerError, ServerResult},
        ServerResponse,
    },
    proto::{
        blockchain::{
            // constants::PUBLIC_KEY_LENGTH,
            result::BlockchainProtoError,
            wallet::PublicKey,
        },
        helpers::hex_to_string::{hex_byte, hex_pubkey, hex_slice},
    },
    NODE_CONFIG,
};

#[derive(Debug)]
pub(super) struct NewPeer;

impl NewPeer {
    pub(super) fn handler(request: &Request) -> ServerResult<ServerResponse> {
        if *request.method() == Method::Get {
            let user_agent = match request
                .headers()
                .get(UserAgent::default().into_header().name())
            {
                Some(header_entry) => header_entry,
                None => return Ok(ServerResponse::new(StatusCode::BadRequest)),
            };

            // TODO get from Authorization
            let client_pubkey_str = "";

            let client_pubkey: PublicKey = client_pubkey_str.parse()?;

            let maybe_user_authorization = request
                .headers()
                .get(Authorization::default().into_header().name());

            println!("NodeServer: (NewPeer): [{}]", user_agent);

            let secret_key = NODE_CONFIG
                .get()
                .and_then(|config| config.secret_key())
                .ok_or(ServerError::NodeSigningKey(
                    "Error on getting signingkey".into(),
                ))?;

            let challenge: [u8; 8] = {
                use rand::{rngs::OsRng, RngCore};
                let mut inner_buf = [0u8; 8];
                OsRng.fill_bytes(&mut inner_buf);
                inner_buf
            };

            let pubkey = PublicKey::from(secret_key);

            let realm_str = format!("{}", client_pubkey);

            let signature = secret_key.sign(&challenge);

            let challenge_str = hex_slice(&challenge);

            let mut to_sign = vec![];
            to_sign.extend(client_pubkey.to_bytes());
            to_sign.extend(challenge);
            // let mut to_sign: [u8; PUBLIC_KEY_LENGTH + 8] = [0; PUBLIC_KEY_LENGTH + 8];
            // to_sign[0..PUBLIC_KEY_LENGTH].copy_from_slice(&client_pubkey.to_bytes());
            // to_sign[PUBLIC_KEY_LENGTH..PUBLIC_KEY_LENGTH + 8].copy_from_slice(&challenge);

            let signature = {
                use base64::{engine::general_purpose, Engine as _};
                let inner_signature = secret_key.sign(&to_sign);
                let b64_outcome = general_purpose::STANDARD.encode(inner_signature.to_bytes());
                b64_outcome
            };

            let header_value = format!(
                r#"PKI realm="{realm_str}", challenge="{challenge_str}", signature="{signature}", public_key="{pubkey}""#,
            );

            let www_authenticate = WWWAuthenticate::new(&header_value)?;

            Ok(ServerResponse::new(StatusCode::Unauthorized).add_header(www_authenticate))
        } else {
            Ok(ServerResponse::new(StatusCode::NotFound))
        }
    }
}
