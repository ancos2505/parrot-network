use crate::proto::{
    blockchain::{
        block::{Block, BlockIndex},
        traits::Serializable,
    },
    // helpers::hex_to_string::hex_slice,
};
use redb::{Database, ReadableTable, TableDefinition};

use super::server::result::{ServerError, ServerResult};

const TABLE: TableDefinition<'_, u64, [u8; Block::PAYLOAD_LEN]> = TableDefinition::new("ledger");
const DB_FILE_PATH: &str = "parrot-ledger.redb";

#[derive(Debug)]
pub(crate) struct ParrotDb {
    db: Database,
    last_id: BlockIndex,
}

impl ParrotDb {
    pub(crate) fn open() -> ServerResult<Self> {
        let (is_created, db) = {
            if let Ok(db) = Database::open(DB_FILE_PATH) {
                (false, db)
            } else {
                let db = Database::create(DB_FILE_PATH)?;
                let write_txn = db.begin_write()?;
                {
                    let mut table = write_txn.open_table(TABLE)?;
                    // TODO
                    table.insert(0, [0; Block::PAYLOAD_LEN])?;
                }
                write_txn.commit()?;
                (true, db)
            }
        };
        let read_txn = db.begin_read()?;
        let table = read_txn.open_table(TABLE)?;

        let last_id = if is_created {
            0
        } else {
            table
                .last()?
                .map(|(k, _)| k.value())
                .ok_or(ServerError::custom("Impossible state on reading last id"))?
        };
        Ok(Self {
            db,
            last_id: BlockIndex::new(last_id),
        })
    }

    pub(crate) fn save_block(&mut self, block: &Block) -> ServerResult<u64> {
        let write_txn = self.db.begin_write()?;

        let new_id = *self.last_id + 1;

        // TODO: Define how a new block will be selected and accepted
        // TODO: Set logic behind new index creation

        // {
        //     let mut table = write_txn.open_table(TABLE)?;
        //     let block_bytes: [u8; Block::PAYLOAD_LEN] = block.serialize_to_bytes()?;

        //     assert_eq!((&block_bytes).len(), Block::PAYLOAD_LEN);
        //     // dbg!(hex_slice(&block_bytes), (&block_bytes).len());
        //     table.insert(new_id, block_bytes)?;
        // }

        write_txn.commit()?;
        self.last_id = BlockIndex::new(new_id);

        Ok(new_id)
    }

    // pub(crate) fn get_block(&self, db_id: u64) -> ServerResult<Block> {
    //     let read_txn = self.db.begin_read()?;
    //     let table = read_txn.open_table(TABLE)?;
    //     let data = table.get(db_id)?.ok_or(ServerError::custom(
    //         "Impossible state on get block from database",
    //     ))?;
    //     let retrieved_block = Block::deserialize_from_bytes(data.value())?;

    //     Ok(retrieved_block)
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::proto::blockchain::traits::Serializable;
    // #[test]
    // fn ok_on_write_and_read_parrot_db_entries() -> ServerResult<()> {
    //     let block_bytes = Block::genesis_block()?.serialize_to_bytes()?;
    //     let db = Database::open(DB_FILE_PATH).or_else(|_| Database::create(DB_FILE_PATH))?;

    //     let write_txn = db.begin_write()?;
    //     for i in 0..100 {
    //         let mut table = write_txn.open_table(TABLE)?;
    //         table.insert(i + 1, block_bytes)?;
    //     }
    //     write_txn.commit()?;

    //     let read_txn = db.begin_read()?;
    //     let table = read_txn.open_table(TABLE)?;
    //     for i in 0..100 {
    //         assert_eq!(table.get(i + 1)?.unwrap().value(), block_bytes);
    //     }

    //     Ok(())
    // }
}
