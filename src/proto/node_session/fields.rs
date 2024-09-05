use std::{fmt::Display, str::FromStr};

use array_bytes::{bytes2hex, hex2array};

use crate::proto::blockchain::result::BlockchainProtoError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Realm {
    ParrotNode,
}
impl Realm {
    pub(crate) fn as_bytes(&self) -> [u8; 2] {
        u16::from(self).to_be_bytes()
    }
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
