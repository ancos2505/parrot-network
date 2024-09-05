pub(crate) mod builder;
pub(crate) mod fields;
use std::{fmt::Display, str::FromStr};

use crate::proto::blockchain::{
    result::BlockchainProtoError,
    wallet::{PublicKey, Signature},
};

use super::fields::Realm;

use self::{builder::PkiClientChallengeBuilder, fields::ClientChallenge};

#[derive(Debug, PartialEq)]
pub(crate) struct PkiClientChallenge {
    realm: Realm,
    challenge: ClientChallenge,
    signature: Signature,
    public_key: PublicKey,
}
impl PkiClientChallenge {
    pub(crate) fn builder() -> PkiClientChallengeBuilder {
        PkiClientChallengeBuilder
    }

    pub(crate) fn realm(&self) -> &Realm {
        &self.realm
    }

    pub(crate) fn challenge(&self) -> &ClientChallenge {
        &self.challenge
    }

    pub(crate) fn signature(&self) -> &Signature {
        &self.signature
    }

    pub(crate) fn public_key(&self) -> &PublicKey {
        &self.public_key
    }

    pub(crate) fn as_bytes(&self) -> Vec<u8> {
        let mut vec = vec![];
        vec.extend(&self.realm.as_bytes());
        vec.extend(self.challenge.as_bytes());
        vec.extend(self.signature.to_bytes());
        vec.extend(self.public_key.to_bytes());
        vec
    }
}

impl FromStr for PkiClientChallenge {
    type Err = BlockchainProtoError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut preamble_iter = input.split("PKI ");
        let _discarded_before_preamble = preamble_iter.next();

        let fields_region = preamble_iter
            .next()
            .ok_or(BlockchainProtoError::PkiChallenge(
                "Invalid Payload for PkiClientChallenge".into(),
            ))?;

        let mut fields_iter = fields_region.split(',');

        let realm: Realm = fields_iter
            .next()
            .ok_or(BlockchainProtoError::PkiChallenge(
                "Not found Payload for PkiClientChallenge.realm".into(),
            ))
            .and_then(|s| {
                s.split_once("=").ok_or(BlockchainProtoError::PkiChallenge(
                    "Invalid Payload for PkiClientChallenge.realm".into(),
                ))
            })
            .and_then(|(_, value)| value.replace("\"", "").trim().parse())?;

        let challenge: ClientChallenge = fields_iter
            .next()
            .ok_or(BlockchainProtoError::PkiChallenge(
                "Not found Payload for PkiClientChallenge.challenge".into(),
            ))
            .and_then(|s| {
                s.split_once("=").ok_or(BlockchainProtoError::PkiChallenge(
                    "Invalid Payload for PkiClientChallenge.challenge".into(),
                ))
            })
            .and_then(|(_, value)| value.replace("\"", "").trim().parse())?;

        let signature: Signature = fields_iter
            .next()
            .ok_or(BlockchainProtoError::PkiChallenge(
                "Not found Payload for PkiClientChallenge.signature".into(),
            ))
            .and_then(|s| {
                s.split_once("=").ok_or(BlockchainProtoError::PkiChallenge(
                    "Invalid Payload for PkiClientChallenge.signature".into(),
                ))
            })
            .and_then(|(_, value)| value.replace("\"", "").trim().parse())?;

        let public_key: PublicKey = fields_iter
            .next()
            .ok_or(BlockchainProtoError::PkiChallenge(
                "Not found Payload for PkiClientChallenge.public_key".into(),
            ))
            .and_then(|s| {
                s.split_once("=").ok_or(BlockchainProtoError::PkiChallenge(
                    "Invalid Payload for PkiClientChallenge.public_key".into(),
                ))
            })
            .and_then(|(_, value)| value.replace("\"", "").trim().parse())?;

        Ok(Self {
            realm,
            challenge,
            signature,
            public_key,
        })
    }
}

impl Display for PkiClientChallenge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            realm,
            challenge,
            signature,
            public_key,
        } = self;
        write!(
            f,
            r#"PKI realm="{realm}", challenge="{challenge}", signature="{signature}", public_key="{public_key}""#
        )
    }
}
