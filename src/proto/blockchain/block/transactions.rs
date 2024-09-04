use crate::proto::blockchain::{
    block::Block,
    // constants::LOCAL_NET_GENESIS_WALLET_SECRET_KEY,
    result::{BlockchainProtoError, BlockchainProtoResult},
    tokens::Wings,
    traits::Serializable,
    transaction::Transaction,
    wallet::Wallet,
};

#[derive(Debug, PartialEq)]
pub(crate) struct BlockTransactions(Vec<Transaction>);

impl BlockTransactions {
    #[cfg(debug_assertions)]
    pub(crate) const TRANSACTIONS: usize = 3;

    #[cfg(not(debug_assertions))]
    pub(crate) const TRANSACTIONS: usize = u8::MAX as usize;

    pub(crate) const PAYLOAD_LEN: usize = Self::TRANSACTIONS * Transaction::PAYLOAD_LEN;

    //     pub(crate) fn genesis_transactions() -> BlockchainProtoResult<Self> {
    //         // TODO
    //         let mut wallet = Wallet::keypair_import(&DEV_TEST_WALLET_KEYPAIR)?;

    //         let to = wallet.pubkey().clone();

    //         let mut transactions: Vec<Transaction> = vec![];

    //         for _ in 0..Self::TRANSACTIONS {
    //             transactions
    //                 .push(wallet.transfer(to.clone(), Wings::new(Block::PARTITION_255_TOKENS))?);
    //         }
    //         if transactions.len() == Self::TRANSACTIONS {
    //             Ok(Self(transactions))
    //         } else {
    //             Err(BlockchainProtoError::custom(
    //                 "Impossible state on generate genesis block transactions",
    //             ))
    //         }
    //     }
}

// impl Serializable for BlockTransactions {
//     type Bytes = [u8; Self::PAYLOAD_LEN];
//     fn serialize_to_bytes(&self) -> BlockchainProtoResult<Self::Bytes> {
//         let mut buf = [0; Self::PAYLOAD_LEN];

//         for (idx_block, transaction) in self.0.iter().enumerate() {
//             let cur_idx = idx_block * Transaction::PAYLOAD_LEN;
//             buf[cur_idx..cur_idx + Transaction::PAYLOAD_LEN]
//                 .copy_from_slice(&transaction.serialize_to_bytes()?);
//         }

//         Ok(buf)
//     }

//     fn deserialize_from_bytes(bytes: Self::Bytes) -> BlockchainProtoResult<Self> {
//         let mut transactions = vec![];

//         let rounds = Self::PAYLOAD_LEN / Transaction::PAYLOAD_LEN;

//         for idx_block in 0..rounds {
//             let cur_idx = idx_block * Transaction::PAYLOAD_LEN;
//             let buf: [u8; Transaction::PAYLOAD_LEN] =
//                 bytes[cur_idx..cur_idx + Transaction::PAYLOAD_LEN].try_into()?;
//             transactions.push(Transaction::deserialize_from_bytes(buf)?);
//         }

//         Ok(Self(transactions))
//     }
// }
