use serde::{Deserialize, Serialize};

use super::{BlockIndex, BlockNonce, BlockPayload, BlockTimestamp};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub(crate) struct BlockHash([u8; 32]);

impl From<[u8; 32]> for BlockHash {
    fn from(value: [u8; 32]) -> Self {
        Self(value)
    }
}

impl BlockHash {
    pub(crate) fn get(&self) -> &[u8; 32] {
        &self.0
    }
    pub(crate) fn zero() -> Self {
        Self([0; 32])
    }
}
