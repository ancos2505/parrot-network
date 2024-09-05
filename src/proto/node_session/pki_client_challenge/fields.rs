use std::{fmt::Display, str::FromStr};

use array_bytes::hex2array;
use falcon_rust::falcon1024;

use crate::proto::{
    blockchain::{
        result::BlockchainProtoError,
        wallet::{PublicKey, Signature},
    },
    helpers::hex_to_string::hex_slice,
};

#[derive(Debug, PartialEq)]
pub(crate) struct ClientChallenge(Box<[u8; 8]>);
impl ClientChallenge {
    pub(crate) fn new() -> Self {
        let challenge: Box<[u8; 8]> = {
            use rand::{rngs::OsRng, RngCore};
            let mut inner_buf = [0u8; 8];
            OsRng.fill_bytes(&mut inner_buf);
            inner_buf.into()
        };
        Self(challenge)
    }
    pub(crate) fn as_bytes(&self) -> &[u8; 8] {
        self.0.as_ref()
    }
    pub(crate) fn verify(&self, signature: &Signature, public_key: &PublicKey) -> bool {
        let sig = signature.as_inner();
        let pk = public_key.as_inner();
        falcon1024::verify(&*self.0, sig, pk)
    }
}

impl FromStr for ClientChallenge {
    type Err = BlockchainProtoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = hex2array(s)?;
        Ok(Self(inner.into()))
    }
}

impl Display for ClientChallenge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex_slice(&*self.0))
    }
}
