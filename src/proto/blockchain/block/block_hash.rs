use std::ops::Deref;

use crate::proto::blockchain::{result::BlockchainProtoResult, traits::Serializable};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct BlockHash([u8; Self::PAYLOAD_LEN]);

impl BlockHash {
    pub(crate) const PAYLOAD_LEN: usize = 32;

    pub(crate) const fn zero() -> Self {
        Self([0; Self::PAYLOAD_LEN])
    }

    pub(crate) fn new(hash: [u8; Self::PAYLOAD_LEN]) -> Self {
        Self(hash)
    }
}

impl Deref for BlockHash {
    type Target = [u8; Self::PAYLOAD_LEN];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Serializable for BlockHash {
    type Bytes = [u8; Self::PAYLOAD_LEN];

    fn serialize_to_bytes(&self) -> BlockchainProtoResult<Self::Bytes> {
        Ok(self.0)
    }
    fn deserialize_from_bytes(bytes: Self::Bytes) -> BlockchainProtoResult<Self> {
        Ok(Self(bytes))
    }
}
