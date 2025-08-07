use crate::{db::utils::get_formated_datetime, entities::product::Product, DB_NAME};
use rusqlite::{Connection, params};
use time::OffsetDateTime;

pub struct FoodIntake {
    pub product_name: String,
    pub calories: f32,
    pub proteins: f32,
    pub fats: f32,
    pub carbohydrates: f32,
    pub weight: i32,
    pub eaten_at: String,
}

impl FoodIntake {
    pub fn new (product: Product, weight: i32) -> Self {
        let current_date_time;

        match OffsetDateTime::now_local() {
            Ok(local_time) => current_date_time = local_time,
            Err(_) => current_date_time = OffsetDateTime::now_utc(),
        }

        let formated_date_time = get_formated_datetime(current_date_time);

        Self {
            product_name: product.name,
            calories: Self::convert_number_to_weight(product.calories, weight),
            proteins: Self::convert_number_to_weight(product.proteins, weight),
            fats: Self::convert_number_to_weight(product.fats, weight),
            carbohydrates: Self::convert_number_to_weight(product.carbohydrates, weight),
            weight: weight,
            eaten_at: formated_date_time
        }
    }

    fn convert_number_to_weight(number: f32, weight: i32) -> f32 {
        (number / 100.0) * weight as f32
    }
}

pub fn insert_food_intake(intake: FoodIntake) -> Result<(), rusqlite::Error> {
    let connection = Connection::open(DB_NAME)?;

    connection.execute(
        "insert into food_intakes (product_name, calories, proteins, fats, carbohydrates, weight, eaten_at) values (?, ?, ?, ?, ?, ?, ?)",
        params![
            intake.product_name,
            intake.calories,
            intake.proteins,
            intake.fats,
            intake.carbohydrates,
            intake.weight,
            intake.eaten_at
        ],
    )?;

    Ok(())
}