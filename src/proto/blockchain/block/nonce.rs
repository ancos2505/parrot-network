use std::ops::Deref;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub(crate) struct BlockNonce(u64);

impl From<u64> for BlockNonce {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl BlockNonce {
    pub(crate) fn zero() -> Self {
        Self(0)
    }
}

impl Deref for BlockNonce {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
