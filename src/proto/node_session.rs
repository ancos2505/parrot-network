pub(crate) mod fields;
pub(crate) mod pki_client_challenge;
pub(crate) mod pki_server_challenge;

use super::blockchain::wallet::{PublicKey, Signature};

// TODO
#[derive(Debug, PartialEq)]
pub struct PkiResponse {
    // Authorization: PKI
    // Client PublicKey
    realm: PublicKey,
    // Received from Server = public_key + signature + challenge
    payload: ServerPayload,
    // Signature (base64 encoded)
    signature: Signature,
    // Server PublicKey
    public_key: PublicKey,
}

// Received from Server = public_key + signature + challenge
#[derive(Debug, PartialEq, Eq)]
pub struct ServerPayload([u8; 104]);
