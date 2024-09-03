use std::{fmt::Display, str::FromStr};

use super::blockchain::{
    result::BlockchainProtoError,
    wallet::{PublicKey, Signature},
};
use array_bytes::{bytes2hex, hex2array};

#[derive(Debug, PartialEq)]
pub struct ServerSideChallenge {
    // WWW-Authenticate: PKI
    // Realm name
    realm: Realm,
    // Challenge
    challenge: Challenge,
    // Signature (base64 encoded)
    signature: Signature,
    // Client PublicKey
    public_key: PublicKey,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Realm {
    ParrotNode,
}

impl TryFrom<u16> for Realm {
    type Error = BlockchainProtoError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::ParrotNode),
            _ => Err(BlockchainProtoError::custom("Invalid payload for Realm")),
        }
    }
}

impl From<&Realm> for u16 {
    fn from(value: &Realm) -> Self {
        match value {
            Realm::ParrotNode => 1,
        }
    }
}

impl FromStr for Realm {
    type Err = BlockchainProtoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 4 {
            let bytes: [u8; 2] = hex2array(s)?;
            let number = u16::from_be_bytes(bytes);
            Ok(number.try_into()?)
        } else {
            Err(BlockchainProtoError::custom(
                "Invalid payload size for Realm.",
            ))
        }
    }
}

impl Display for Realm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", bytes2hex("", u16::from(self).to_be_bytes()))
    }
}

// impl FromStr for PkiSyn {
//     type Err = BlockchainProtoError;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let x = Signature::from_bytes();
//     }
// }

#[derive(Debug, PartialEq)]
pub struct Challenge([u8; 8]);

#[derive(Debug, PartialEq)]
pub struct ClientSideChallengeResponse {
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
