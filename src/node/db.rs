use crate::{
    node::webui::ServerResult,
    proto::blockchain::block::{Block, BlockIndex},
};
use redb::{Database, ReadableTable, TableDefinition};

use super::webui::ServerError;

const TABLE: TableDefinition<'_, u64, &str> = TableDefinition::new("ledger");

#[derive(Debug)]
pub(crate) struct ParrotDb {
    db: Database,
    last_id: BlockIndex,
}

impl ParrotDb {
    pub(crate) fn open() -> ServerResult<Self> {
        let (is_created, db) = {
            let db_file_path = "parrot-ledger.redb";
            if let Ok(db) = Database::open(db_file_path) {
                (false, db)
            } else {
                let db = Database::create(db_file_path)?;
                let write_txn = db.begin_write()?;
                {
                    let mut table = write_txn.open_table(TABLE)?;
                    table.insert(0, "{}")?;
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
            last_id: last_id.into(),
        })
    }

    pub(crate) fn save_block(&mut self, block: Block) -> ServerResult<u64> {
        let write_txn = self.db.begin_write()?;

        let new_id = *self.last_id + 1;

        // TODO: Define how a new block will be selected and accepted
        // TODO: Set logic behind new index creation

        {
            let mut table = write_txn.open_table(TABLE)?;
            table.insert(new_id, serde_json::to_string(&block)?.as_str())?;
        }

        write_txn.commit()?;
        self.last_id = new_id.into();

        Ok(new_id)
    }

    pub(crate) fn get_block(&self, block_index: u64) -> ServerResult<Block> {
        dbg!(block_index);
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(TABLE)?;
        let data = table.get(block_index)?.ok_or(ServerError::custom(
            "Impossible state on get block from database",
        ))?;

        Ok(serde_json::from_str::<Block>(data.value())?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_on_write_and_read_parrot_db_entries() -> ServerResult<()> {
        let db_file_path = "parrot-db.redb";

        let db = Database::open(db_file_path).or_else(|_| Database::create(db_file_path))?;

        let write_txn = db.begin_write()?;
        for i in 0..100 {
            let mut table = write_txn.open_table(TABLE)?;
            table.insert(
                i + 1,
                format!(r#"{{ "{}": "adsdasdasdasd" }}"#, i + 1).as_str(),
            )?;
        }
        write_txn.commit()?;

        let read_txn = db.begin_read()?;
        let table = read_txn.open_table(TABLE)?;
        for i in 0..100 {
            assert_eq!(
                table.get(i + 1)?.unwrap().value(),
                format!(r#"{{ "{}": "adsdasdasdasd" }}"#, i + 1).as_str(),
            );
        }

        Ok(())
    }
}
