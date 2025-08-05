use rusqlite::{Connection, params};
use crate::DB_NAME;

pub struct FoodIntake {
    pub id: i32,
    pub product_name: String,
    pub eaten_at: String,
    pub weight: i32,
}

pub fn insert_food_intake(product_name: String, eaten_at: String, weight: i32) -> Result<(), rusqlite::Error> {
    let connection = Connection::open(DB_NAME)?;

    connection.execute(
        "insert into food_intakes (product_name, eaten_at, weight) values (?, ?, ?)",
        params![product_name, eaten_at, weight],
    )?;

    Ok(())
}