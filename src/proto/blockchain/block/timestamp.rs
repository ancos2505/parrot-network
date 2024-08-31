use std::ops::Deref;

use crate::proto::blockchain::{result::BlockchainProtoResult, traits::Serializable};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct BlockTimestamp(u32);

impl Deref for BlockTimestamp {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl BlockTimestamp {
    pub(crate) const PAYLOAD_LEN: usize = 4;

    pub(crate) const fn genesis() -> Self {
        Self(1725000000)
    }
    pub(crate) fn now() -> BlockchainProtoResult<Self> {
        use std::time::SystemTime;
        let now = SystemTime::now();
        let now_unix_epoch = now.duration_since(SystemTime::UNIX_EPOCH)?;

        Ok(Self(now_unix_epoch.as_secs().try_into()?))
    }
}

impl Serializable for BlockTimestamp {
    type Bytes = [u8; Self::PAYLOAD_LEN];
    fn serialize_to_bytes(&self) -> BlockchainProtoResult<Self::Bytes> {
        Ok(self.0.to_be_bytes())
    }
    fn deserialize_from_bytes(bytes: Self::Bytes) -> BlockchainProtoResult<Self> {
        Ok(Self(u32::from_be_bytes(bytes)))
    }
}
