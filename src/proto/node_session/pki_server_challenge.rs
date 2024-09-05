pub(crate) mod builder;
pub(crate) mod fields;

use std::{fmt::Display, str::FromStr};

use crate::proto::blockchain::{
    result::BlockchainProtoError,
    wallet::{PublicKey, Signature},
};

use super::fields::Realm;

use self::{builder::PkiServerChallengeBuilder, fields::ServerChallenge};

#[derive(Debug, PartialEq)]
pub(crate) struct PkiServerChallenge {
    realm: Realm,
    challenge: ServerChallenge,
    signature: Signature,
    public_key: PublicKey,
}
impl PkiServerChallenge {
    pub(crate) fn builder() -> PkiServerChallengeBuilder {
        PkiServerChallengeBuilder
    }

    pub(crate) fn realm(&self) -> &Realm {
        &self.realm
    }

    pub(crate) fn challenge(&self) -> &ServerChallenge {
        &self.challenge
    }

    pub(crate) fn signature(&self) -> &Signature {
        &self.signature
    }

    pub(crate) fn public_key(&self) -> &PublicKey {
        &self.public_key
    }
}

impl FromStr for PkiServerChallenge {
    type Err = BlockchainProtoError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut preamble_iter = input.split("PKI ");
        let _discarded_before_preamble = preamble_iter.next();

        let fields_region = preamble_iter
            .next()
            .ok_or(BlockchainProtoError::PkiChallenge(
                "Invalid Payload for PkiServerChallenge".into(),
            ))?;

        let mut fields_iter = fields_region.split(',');

        let realm: Realm = fields_iter
            .next()
            .ok_or(BlockchainProtoError::PkiChallenge(
                "Not found Payload for PkiServerChallenge.realm".into(),
            ))
            .and_then(|s| {
                s.split_once("=").ok_or(BlockchainProtoError::PkiChallenge(
                    "Invalid Payload for PkiServerChallenge.realm".into(),
                ))
            })
            .and_then(|(_, value)| value.replace("\"", "").trim().parse())?;

        let challenge: ServerChallenge = fields_iter
            .next()
            .ok_or(BlockchainProtoError::PkiChallenge(
                "Not found Payload for PkiServerChallenge.challenge".into(),
            ))
            .and_then(|s| {
                s.split_once("=").ok_or(BlockchainProtoError::PkiChallenge(
                    "Invalid Payload for PkiServerChallenge.challenge".into(),
                ))
            })
            .and_then(|(_, value)| value.replace("\"", "").trim().parse())?;

        let signature: Signature = fields_iter
            .next()
            .ok_or(BlockchainProtoError::PkiChallenge(
                "Not found Payload for PkiServerChallenge.signature".into(),
            ))
            .and_then(|s| {
                s.split_once("=").ok_or(BlockchainProtoError::PkiChallenge(
                    "Invalid Payload for PkiServerChallenge.signature".into(),
                ))
            })
            .and_then(|(_, value)| value.replace("\"", "").trim().parse())?;

        let public_key: PublicKey = fields_iter
            .next()
            .ok_or(BlockchainProtoError::PkiChallenge(
                "Not found Payload for PkiServerChallenge.public_key".into(),
            ))
            .and_then(|s| {
                s.split_once("=").ok_or(BlockchainProtoError::PkiChallenge(
                    "Invalid Payload for PkiServerChallenge.public_key".into(),
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

impl Display for PkiServerChallenge {
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
