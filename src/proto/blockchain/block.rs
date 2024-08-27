mod block_hash;
mod index;
mod nonce;
mod payload;
mod timestamp;

use self::{
    block_hash::BlockHash, index::BlockIndex, nonce::BlockNonce, payload::BlockPayload,
    timestamp::BlockTimestamp,
};

pub(crate) struct Block {
    current: BlockHash,
    index: BlockIndex,
    nonce: BlockNonce,
    payload: BlockPayload,
    previous: BlockHash,
    timestamp: BlockTimestamp,
}
