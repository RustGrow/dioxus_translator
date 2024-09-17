use redb::{AccessGuard, Database, Error, ReadableTable, TableDefinition};

const OPTIONS: TableDefinition<&str, &str> = TableDefinition::new("options");

pub fn insert_to_localdb(k: &str, v: &str) -> Result<(), Error> {
    let db = Database::create("db/local_db.redb")?;
    let write_txn = db.begin_write()?;
    {
        let mut table = write_txn.open_table(OPTIONS).unwrap();
        table.insert(k, v).unwrap();
    }
    write_txn.commit()?;
    Ok(())
}

pub fn get_from_localdb(k: &str) -> Option<AccessGuard<'_, &str>> {
    let db = Database::create("db/local_db.redb").unwrap();
    let read_txn = db.begin_read().unwrap();
    let table = read_txn.open_table(OPTIONS).unwrap();

    // let val = table.get(k).unwrap().expect("msg").value();
    // let binding: Option<AccessGuard<'_, &str>> = table.get(k).unwrap()
    table.get(k).unwrap()
    // binding.value().to_owned()
}
