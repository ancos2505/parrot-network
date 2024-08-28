mod block_hash;
mod index;
mod nonce;
mod payload;
mod timestamp;

use serde::{Deserialize, Serialize};

use crate::node::webui::{ServerError, ServerResult};

pub(crate) use self::{
    block_hash::BlockHash, index::BlockIndex, nonce::BlockNonce, payload::BlockPayload,
    timestamp::BlockTimestamp,
};

use super::constants::BLOCK_PAYLOAD_LEN;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Block {
    pub previous: BlockHash,
    pub index: BlockIndex,
    pub payload: BlockPayload,
    pub timestamp: BlockTimestamp,
    pub nonce: BlockNonce,
    pub current: BlockHash,
}
impl Block {
    pub(crate) fn genesis_block() -> ServerResult<Self> {
        let mut buf = Box::new([0; BLOCK_PAYLOAD_LEN]);
        let genesis_info = b"Genesis Block";
        let gen_length = genesis_info.len();
        for (idx, c) in genesis_info.iter().enumerate() {
            buf[(BLOCK_PAYLOAD_LEN - gen_length) + idx] = *c;
        }
        let payload = BlockPayload::from(buf);
        let previous = BlockHash::zero();
        let timestamp = BlockTimestamp::now()?;

        let index = BlockIndex::zero();
        let nonce = BlockNonce::zero();
        let current = BlockHash::zero();

        Ok(Self {
            current,
            index,
            nonce,
            payload,
            previous,
            timestamp,
        })
    }
    pub(crate) fn bytes_to_mine(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(*self.previous.get());
        bytes.extend((*self.index).to_be_bytes());
        bytes.extend(self.payload.get());
        bytes.extend((*self.timestamp).to_be_bytes());
        bytes.extend((*self.nonce).to_be_bytes());

        bytes
    }
    pub(crate) fn mine(&mut self) -> ServerResult<()> {
        use sha2::{Digest, Sha256};
        use std::time::Instant;
        let now = Instant::now();

        for nonce_attempt in 0..u64::MAX {
            self.nonce = nonce_attempt.into();

            let bytes = self.bytes_to_mine();

            let mut hasher = Sha256::new();

            hasher.update(bytes);

            let hash: [u8; 32] = hasher.finalize().into();

            let block_hash = BlockHash::from(hash);

            if is_nonce_found(&block_hash) {
                self.current = block_hash;
                println!("Mined in {} secs", now.elapsed().as_secs_f32());
                return Ok(());
            } else {
                continue;
            }
        }
        Err(ServerError::custom("Impossible state on mining new block"))
    }
}

#[cfg(debug_assertions)]
fn is_nonce_found(block_hash: &BlockHash) -> bool {
    // const DIFFICULTY: u32 = 0x00_FF_FF_FF_FF;
    let hash = block_hash.get();
    match (hash.get(0), hash.get(1)) {
        (Some(0), Some(0)) => true,
        _ => false,
    }
}

#[cfg(not(debug_assertions))]
fn is_nonce_found(block_hash: &BlockHash) -> bool {
    // const DIFFICULTY: u32 = 0x00_0F_FF_FF_FF;
    let hash = block_hash.get();
    match (hash.get(0), hash.get(1), hash.get(2)) {
        (Some(0), Some(0), Some(0)) => true,
        _ => false,
    }
}
