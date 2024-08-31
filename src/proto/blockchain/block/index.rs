use std::ops::Deref;

use crate::proto::blockchain::{result::BlockchainProtoResult, traits::Serializable};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct BlockIndex(u64);

impl BlockIndex {
    pub(crate) const PAYLOAD_LEN: usize = 8;

    pub(crate) const fn zero() -> Self {
        Self(0)
    }

    pub(crate) fn new(index: u64) -> Self {
        Self(index)
    }
}

impl Deref for BlockIndex {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Serializable for BlockIndex {
    type Bytes = [u8; Self::PAYLOAD_LEN];
    fn serialize_to_bytes(&self) -> BlockchainProtoResult<Self::Bytes> {
        Ok(self.0.to_be_bytes())
    }
    fn deserialize_from_bytes(bytes: Self::Bytes) -> BlockchainProtoResult<Self> {
        Ok(Self(u64::from_be_bytes(bytes)))
    }
}
