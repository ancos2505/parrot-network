use crate::node::webui::ServerResult;
use redb::{Database, Error, ReadableTable, TableDefinition};

const TABLE: TableDefinition<'_, u64, &str> = TableDefinition::new("parrot-ledger");
struct ParrotDb;

impl ParrotDb {}

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
