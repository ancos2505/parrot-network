use std::{fmt::Display, str::FromStr};

use array_bytes::hex2bytes;
use falcon_rust::falcon1024;

use crate::proto::{
    blockchain::{
        result::BlockchainProtoError,
        wallet::{PublicKey, Signature},
    },
    helpers::hex_to_string::hex_slice,
    node_session::pki_client_challenge::PkiClientChallenge,
};

#[derive(Debug, PartialEq)]
pub(crate) struct ServerChallenge(Vec<u8>);
impl ServerChallenge {
    pub(crate) fn new(client_pki_challenge: &PkiClientChallenge) -> Self {
        use rand::{rngs::OsRng, RngCore};

        let mut challenge = client_pki_challenge.as_bytes();

        let mut nonce = [0u8; 8];

        OsRng.fill_bytes(&mut nonce);

        challenge.extend(nonce);

        Self(challenge)
    }
    pub(crate) fn as_bytes(&self) -> &[u8] {
        &self.0
    }
    pub(crate) fn verify(&self, signature: &Signature, public_key: &PublicKey) -> bool {
        let sig = signature.as_inner();
        let pk = public_key.as_inner();
        falcon1024::verify(&*self.0, sig, pk)
    }
}

impl FromStr for ServerChallenge {
    type Err = BlockchainProtoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = hex2bytes(s)?;
        Ok(Self(inner.into()))
    }
}

impl Display for ServerChallenge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex_slice(&*self.0))
    }
}
