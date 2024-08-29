use ed25519_dalek::{Signature, SigningKey};

use crate::proto::blockchain::wallet::PublicKey;

pub(crate) fn hex_slice(bytes: &[u8]) -> String {
    let mut output = "".to_string();
    for n in bytes {
        output.push_str(format!("{:02x}", n).as_str())
    }
    output
}

pub(crate) fn hex_array_64(bytes: &[u8; 32]) -> String {
    let mut output = "".to_string();
    for n in bytes {
        output.push_str(format!("{:02x}", n).as_str())
    }
    output
}

pub(crate) fn hex_array_32(bytes: &[u8; 32]) -> String {
    let mut output = "".to_string();
    for n in bytes {
        output.push_str(format!("{:02x}", n).as_str())
    }
    output
}

pub(crate) fn hex_pubkey(pubkey: &PublicKey) -> String {
    let mut output = "".to_string();
    for n in **pubkey {
        output.push_str(format!("{:02x}", n).as_str())
    }
    output
}

pub(crate) fn hex_signing_key(signing_key: &SigningKey) -> String {
    let mut output = "".to_string();
    for n in signing_key.to_keypair_bytes() {
        output.push_str(format!("{:02x}", n).as_str())
    }
    output
}

pub(crate) fn hex_signature(signature: &Signature) -> String {
    let mut output = "".to_string();
    for n in signature.to_bytes() {
        output.push_str(format!("{:02x}", n).as_str())
    }
    output
}
