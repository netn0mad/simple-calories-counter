use rusqlite::{Connection, Result};

use crate::DB_NAME;

pub fn init() -> Result<()> {
    let connection = Connection::open(DB_NAME)?;

    connection.execute(
        "create table if not exists food_intakes (
            id integer primary key,
            product_name text not null,
            weight integer not null,
            eaten_at text not null
            ADD CALORIEST, FAT< PROTEIN AND OTHER
        )",
        [],
    )?;

    Ok(())
}