use crate::proto::blockchain::wallet::{PublicKey, SecretKey, Signature};

pub(crate) fn hex_byte(byte: u8) -> String {
    hex_slice(&[byte])
}

pub(crate) fn hex_pubkey(pubkey: &PublicKey) -> String {
    hex_slice(&pubkey.to_bytes())
}

pub(crate) fn hex_secret_key(secret_key: &SecretKey) -> String {
    hex_slice(&secret_key.to_bytes())
}

pub(crate) fn hex_signature(signature: &Signature) -> String {
    hex_slice(&signature.to_bytes())
}

pub(crate) fn hex_slice(bytes: &[u8]) -> String {
    let mut output = "".to_string();
    for n in bytes {
        output.push_str(format!("{:02x}", n).as_str())
    }
    output
}
