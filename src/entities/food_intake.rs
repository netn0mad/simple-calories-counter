use rusqlite::{Connection, params};
use crate::DB_NAME;

pub struct FoodIntake {
    pub product_name: String,
    pub calories: i32,
    pub proteins: f32,
    pub fats: f32,
    pub carbohydrates: f32,
    pub weight: i32,
    pub eaten_at: String,
}

pub fn insert_food_intake(intake: FoodIntake) -> Result<(), rusqlite::Error> {
    let connection = Connection::open(DB_NAME)?;

    connection.execute(
        "insert into food_intakes (product_name, calories, proteins, fats, carbohydrates, weight, eaten_at) values (?, ?, ?, ?, ?, ?, ?)",
        params![intake.product_name, intake.calories, intake.proteins, intake.fats, intake.carbohydrates, intake.weight, intake.eaten_at],
    )?;

    Ok(())
}