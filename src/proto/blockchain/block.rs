mod block_hash;
mod index;
mod nonce;
mod status;
mod timestamp;
mod transactions;

use transactions::BlockTransactions;

use crate::proto::blockchain::{result::BlockchainProtoError, traits::Serializable};

pub(crate) use self::{
    block_hash::BlockHash, index::BlockIndex, nonce::BlockNonce, status::BlockStatus,
    timestamp::BlockTimestamp,
};

use super::result::BlockchainProtoResult;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Block {
    pub status: BlockStatus,
    pub previous: BlockHash,
    pub index: BlockIndex,
    pub transactions: BlockTransactions,
    pub timestamp: BlockTimestamp,
    pub nonce: BlockNonce,
    pub current: BlockHash,
}
impl Block {
    const PARTITION_255_TOKENS: u64 = 72340172838076673;

    pub(crate) const PAYLOAD_LEN: usize = BlockStatus::PAYLOAD_LEN
        + BlockHash::PAYLOAD_LEN
        + BlockIndex::PAYLOAD_LEN
        + BlockTransactions::PAYLOAD_LEN
        + BlockTimestamp::PAYLOAD_LEN
        + BlockNonce::PAYLOAD_LEN
        + BlockHash::PAYLOAD_LEN;

    const PAYLOAD_TO_MINING_LEN: usize = BlockHash::PAYLOAD_LEN
        + BlockIndex::PAYLOAD_LEN
        + BlockTransactions::PAYLOAD_LEN
        + BlockTimestamp::PAYLOAD_LEN
        + BlockNonce::PAYLOAD_LEN;

    pub(crate) fn genesis_block() -> BlockchainProtoResult<Self> {
        let transactions = BlockTransactions::genesis_transactions()?;
        let previous = const { BlockHash::zero() };
        let timestamp = const { BlockTimestamp::genesis() };
        let index = const { BlockIndex::zero() };
        let nonce = const { BlockNonce::zero() };
        let current = const { BlockHash::zero() };

        Ok(Self {
            status: BlockStatus::NotMined,
            previous,
            index,
            transactions,
            timestamp,
            nonce,
            current,
        })
    }

    pub(crate) fn as_bytes_to_mine(
        &self,
    ) -> BlockchainProtoResult<Box<[u8; Self::PAYLOAD_TO_MINING_LEN]>> {
        let mut buf = Box::new([0u8; Self::PAYLOAD_TO_MINING_LEN]);

        const PREVIOUS_END: usize = size_of::<BlockHash>();

        buf[0..PREVIOUS_END].copy_from_slice(&*self.previous);

        const INDEX_END: usize = PREVIOUS_END + size_of::<BlockIndex>();

        buf[PREVIOUS_END..INDEX_END].copy_from_slice(&(*self.index).to_be_bytes());

        const TRANSACTIONS_END: usize = INDEX_END + BlockTransactions::PAYLOAD_LEN;

        buf[INDEX_END..TRANSACTIONS_END].copy_from_slice(&self.transactions.serialize_to_bytes()?);

        const TIMESTAMP_END: usize = TRANSACTIONS_END + size_of::<BlockTimestamp>();
        buf[TRANSACTIONS_END..TIMESTAMP_END].copy_from_slice(&(*self.timestamp).to_be_bytes());
        const NONCE_END: usize = TIMESTAMP_END + size_of::<BlockNonce>();
        buf[TIMESTAMP_END..NONCE_END].copy_from_slice(&(*self.nonce).to_be_bytes());

        Ok(buf)
    }

    pub(crate) fn mine(&mut self) -> BlockchainProtoResult<()> {
        use sha2::{Digest, Sha256};
        use std::time::Instant;

        println!("Mining {} Bytes ...", self.as_bytes_to_mine()?.len());

        let now = Instant::now();

        for nonce_attempt in 0..u64::MAX {
            self.nonce = BlockNonce::new(nonce_attempt);

            let mut hasher = Sha256::new();

            let bytes = self.as_bytes_to_mine()?;

            hasher.update(&*bytes);

            let hash: [u8; 32] = hasher.finalize().into();

            let block_hash = BlockHash::new(hash);

            if is_nonce_found(&block_hash) {
                self.current = block_hash;
                self.status = BlockStatus::Mined;
                println!("Mined in {} secs", now.elapsed().as_secs_f32());
                return Ok(());
            } else {
                continue;
            }
        }
        Err(BlockchainProtoError::custom(
            "Impossible state on mining new block",
        ))
    }

    pub(crate) fn verify(&self) -> BlockchainProtoResult<()> {
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();

        let bytes = self.as_bytes_to_mine()?;

        hasher.update(&*bytes);

        let hash: [u8; 32] = hasher.finalize().into();

        let block_hash = BlockHash::new(hash);

        // dbg!(hex_array_32(&self.current), hex_array_32(&block_hash));

        if self.current == block_hash {
            Ok(())
        } else {
            Err(BlockchainProtoError::custom(
                "Impossible state on mining new block",
            ))
        }
    }
}

impl Serializable<517> for Block {
    fn serialize_to_bytes(&self) -> BlockchainProtoResult<[u8; Self::PAYLOAD_LEN]> {
        let Self {
            status,
            previous,
            index,
            transactions,
            timestamp,
            nonce,
            current,
        } = self;

        let mut buf = [0u8; Self::PAYLOAD_LEN];

        const STATUS_END: usize = size_of::<BlockStatus>();

        buf[0..STATUS_END].copy_from_slice(&status.serialize_to_bytes()?);

        const PREVIOUS_END: usize = STATUS_END + size_of::<BlockHash>();

        buf[STATUS_END..PREVIOUS_END].copy_from_slice(&previous.serialize_to_bytes()?);

        const INDEX_END: usize = PREVIOUS_END + size_of::<BlockIndex>();

        buf[PREVIOUS_END..INDEX_END].copy_from_slice(&index.to_be_bytes());

        const TRANSACTIONS_END: usize = INDEX_END + BlockTransactions::PAYLOAD_LEN;

        buf[INDEX_END..TRANSACTIONS_END].copy_from_slice(&transactions.serialize_to_bytes()?);

        const TIMESTAMP_END: usize = TRANSACTIONS_END + size_of::<BlockTimestamp>();

        buf[TRANSACTIONS_END..TIMESTAMP_END].copy_from_slice(&(*timestamp).to_be_bytes());

        const NONCE_END: usize = TIMESTAMP_END + size_of::<BlockNonce>();

        buf[TIMESTAMP_END..NONCE_END].copy_from_slice(&(*nonce).to_be_bytes());

        const CURRENT_END: usize = NONCE_END + size_of::<BlockHash>();

        buf[NONCE_END..CURRENT_END].copy_from_slice(&current.serialize_to_bytes()?);

        // TODO
        assert_eq!(CURRENT_END, Self::PAYLOAD_LEN);

        Ok(buf)
    }

    fn deserialize_from_bytes(bytes: [u8; Self::PAYLOAD_LEN]) -> BlockchainProtoResult<Self> {
        const STATUS_END: usize = size_of::<BlockStatus>();

        let status = BlockStatus::deserialize_from_bytes([bytes[0]])?;

        const PREVIOUS_END: usize = STATUS_END + size_of::<BlockHash>();

        let previous =
            BlockHash::deserialize_from_bytes(bytes[STATUS_END..PREVIOUS_END].try_into()?)?;

        const INDEX_END: usize = PREVIOUS_END + size_of::<BlockIndex>();

        let index = BlockIndex::deserialize_from_bytes(bytes[PREVIOUS_END..INDEX_END].try_into()?)?;

        const TRANSACTIONS_END: usize = INDEX_END + BlockTransactions::PAYLOAD_LEN;

        let transactions = BlockTransactions::deserialize_from_bytes(
            bytes[INDEX_END..TRANSACTIONS_END].try_into()?,
        )?;

        const TIMESTAMP_END: usize = TRANSACTIONS_END + size_of::<BlockTimestamp>();

        let timestamp = BlockTimestamp::deserialize_from_bytes(
            bytes[TRANSACTIONS_END..TIMESTAMP_END].try_into()?,
        )?;

        const NONCE_END: usize = TIMESTAMP_END + size_of::<BlockNonce>();

        let nonce =
            BlockNonce::deserialize_from_bytes(bytes[TRANSACTIONS_END..TIMESTAMP_END].try_into()?)?;

        const CURRENT_END: usize = NONCE_END + size_of::<BlockHash>();

        let current = BlockHash::deserialize_from_bytes(bytes[NONCE_END..CURRENT_END].try_into()?)?;

        Ok(Self {
            status,
            previous,
            index,
            transactions,
            timestamp,
            nonce,
            current,
        })
    }
}

#[cfg(debug_assertions)]
fn is_nonce_found(block_hash: &BlockHash) -> bool {
    // const DIFFICULTY: u32 = 0x00_FF_FF_FF_FF;
    let hash = &**block_hash;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genesis_block_creation() {
        // Test the creation of a genesis block and validate its fields
        let genesis_block = Block::genesis_block();

        // Check that genesis_block returns Ok
        assert!(genesis_block.is_ok());

        // Unwrap the genesis block
        let block = genesis_block.unwrap();

        // Validate that the block fields are correctly initialized
        assert_eq!(block.previous, BlockHash::zero());
        assert_eq!(block.index, BlockIndex::zero());
        assert_eq!(block.timestamp, BlockTimestamp::genesis());
        assert_eq!(block.nonce, BlockNonce::zero());
        assert_eq!(block.current, BlockHash::zero());

        // Check that transactions are the genesis transactions
        let expected_transactions = BlockTransactions::genesis_transactions().unwrap();
        assert_eq!(block.transactions, expected_transactions);

        // Check that timestamp is correctly set (not necessarily zero)
        assert!(*block.timestamp > 0);
    }

    #[test]
    fn test_block_equality() {
        // Create two identical blocks and ensure they are equal
        let block1 = Block::genesis_block().unwrap();
        let block2 = Block::genesis_block().unwrap();

        // Blocks should be equal since genesis block is deterministic
        assert_eq!(block1, block2);
    }

    #[test]
    fn test_block_inequality() {
        // Create two blocks with different timestamps or transactions
        let block1 = Block::genesis_block().unwrap();
        let mut block2 = Block::genesis_block().unwrap();

        // Modify block2's nonce
        block2.nonce = BlockNonce::new(1);

        // Blocks should not be equal due to different nonces
        assert_ne!(block1, block2);
    }

    #[test]
    fn test_edge_case_large_block_index() {
        // Test block creation with a very large block index (near max value of BlockIndex)
        let mut block = Block::genesis_block().unwrap();
        block.index = BlockIndex::new(u64::MAX);
        assert_eq!(*block.index, u64::MAX);
    }
    #[test]
    fn test_serializable_trait_impl() -> BlockchainProtoResult<()> {
        let mut genesis_block = Block::genesis_block()?;

        genesis_block.mine()?;

        genesis_block.verify()?;

        let bytes = genesis_block.serialize_to_bytes()?;

        let deserialized_block = Block::deserialize_from_bytes(bytes)?;

        assert_eq!(&genesis_block, &deserialized_block);
        Ok(())
    }
}
