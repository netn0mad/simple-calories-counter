use rusqlite::{Connection, Result};

use crate::DB_NAME;

pub fn init() -> Result<()> {
    let connection = Connection::open(DB_NAME)?;

    connection.execute(
        "create table if not exists products (
            id integer primary key,
            name text unique not null,
            calories integer not null,
            protein real not null,
            fat real not null,
            carbohydrates real not null
         )",
        [],
    )?;

    connection.execute(
        "create table if not exists food_intakes (
            id integer primary key,
            product_id integer not null,
            eaten_at text not null,
            weight integer not null,
            foreign key (product_id) references products (id)
        )",
        [],
    )?;

    Ok(())
}