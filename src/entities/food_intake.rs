use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct FoodIntake {
    id: i32,
    product_id: i32,
    eaten_at: String,
    weight: i32,
}

pub fn insert_food_intake(food_intake: FoodIntake) -> Result<(), rusqlite::Error> {
    let connection = Connection::open(DB_NAME)?;

    connection.execute(
        "insert into food_intakes (product_id, eaten_at, weight) values (?, ?, ?)",
        [food_intake.product_id, food_intake.eaten_at, food_intake.weight],
    )?;

    Ok(())
}