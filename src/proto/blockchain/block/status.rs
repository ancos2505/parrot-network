use crate::proto::blockchain::{
    result::{BlockchainProtoError, BlockchainProtoResult},
    traits::Serializable,
};

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum BlockStatus {
    NotMined,
    Mined,
    SentToBeConfirmed,
    BeenConfirmed,
    Confirmed,
}

impl BlockStatus {
    pub(crate) const PAYLOAD_LEN: usize = 1;

    pub(crate) fn zero() -> Self {
        Self::NotMined
    }
}

impl Serializable<1> for BlockStatus {
    fn serialize_to_bytes(&self) -> BlockchainProtoResult<[u8; Self::PAYLOAD_LEN]> {
        let arr = match self {
            BlockStatus::NotMined => [0],
            BlockStatus::Mined => [1],
            BlockStatus::SentToBeConfirmed => [2],
            BlockStatus::BeenConfirmed => [3],
            BlockStatus::Confirmed => [4],
        };
        Ok(arr)
    }

    fn deserialize_from_bytes(bytes: [u8; Self::PAYLOAD_LEN]) -> BlockchainProtoResult<Self> {
        let block_status = match bytes[0] {
            0 => BlockStatus::NotMined,
            1 => BlockStatus::Mined,
            2 => BlockStatus::SentToBeConfirmed,
            3 => BlockStatus::BeenConfirmed,
            4 => BlockStatus::Confirmed,
            _ => {
                return Err(BlockchainProtoError::custom(
                    "Impossible state on deserialize BlockStatus",
                ))
            }
        };

        Ok(block_status)
    }
}
