use std::ops::Deref;

use serde::{Deserialize, Serialize};

use super::Block;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct BlockIndex(u64);

impl BlockIndex {
    pub(crate) fn zero() -> Self {
        Self(0)
    }
}
impl From<u64> for BlockIndex {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl Deref for BlockIndex {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
