use std::ops::Deref;

use crate::proto::blockchain::{result::BlockchainProtoResult, traits::Serializable};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct BlockNonce(u64);

impl BlockNonce {
    pub(crate) const PAYLOAD_LEN: usize = 8;

    pub(crate) const fn zero() -> Self {
        Self(0)
    }

    pub(crate) fn new(nonce: u64) -> Self {
        Self(nonce)
    }
}

impl Deref for BlockNonce {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Serializable for BlockNonce {
    type Bytes = [u8; Self::PAYLOAD_LEN];
    fn serialize_to_bytes(&self) -> BlockchainProtoResult<Self::Bytes> {
        Ok(self.0.to_be_bytes())
    }

    fn deserialize_from_bytes(bytes: Self::Bytes) -> BlockchainProtoResult<Self> {
        Ok(Self(u64::from_be_bytes(bytes)))
    }
}
