use redb::{Database, Error, ReadableTable, TableDefinition};

const OPTIONS: TableDefinition<&str, &str> = TableDefinition::new("options");
