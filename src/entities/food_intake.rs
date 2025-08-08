use crate::{db::utils::{get_current_datetime, get_formated_datetime, get_formated_datetime_to_day}, entities::product::Product, DB_NAME};
use rusqlite::{Connection, params};
use time::OffsetDateTime;
use std::ops::Add;

#[derive(Clone)]
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
        Self {
            product_name: product.name,
            calories: Self::convert_number_to_weight(product.calories, weight),
            proteins: Self::convert_number_to_weight(product.proteins, weight),
            fats: Self::convert_number_to_weight(product.fats, weight),
            carbohydrates: Self::convert_number_to_weight(product.carbohydrates, weight),
            weight: weight,
            eaten_at: get_formated_datetime(get_current_datetime())
        }
    }

    pub fn new_empty() -> Self {
        Self {
            product_name: "".to_string(),
            calories: 0.0,
            proteins: 0.0,
            fats: 0.0,
            carbohydrates: 0.0,
            weight: 0,
            eaten_at: get_formated_datetime_to_day(get_current_datetime())
        }
    }

    fn convert_number_to_weight(number: f32, weight: i32) -> f32 {
        (number / 100.0) * weight as f32
    }
}

impl Add for FoodIntake {
    type Output = FoodIntake;

    fn add(self, other: FoodIntake) -> FoodIntake {
        FoodIntake {
            product_name: self.product_name,
            calories: self.calories + other.calories,
            proteins: self.proteins + other.proteins,
            fats: self.fats + other.fats,
            carbohydrates: self.carbohydrates + other.carbohydrates,
            weight: self.weight + other.weight,
            eaten_at: self.eaten_at,
        }
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

pub fn get_data_for_day(date: OffsetDateTime) -> Result<Vec<FoodIntake>, rusqlite::Error> {
    let connection = Connection::open(DB_NAME)?;
    let mut stmt = connection.prepare("SELECT * FROM food_intakes WHERE strftime('%Y-%m-%d', eaten_at) = ?1")?;

    let iter = stmt.query_map([get_formated_datetime_to_day(date)], |row| {
        Ok(FoodIntake {
            product_name: row.get(1)?,
            calories: row.get(2)?,
            proteins: row.get(3)?,
            fats: row.get(4)?,
            carbohydrates: row.get(5)?,
            weight: row.get(6)?,
            eaten_at: row.get(7)?,
        })
    })?;

    let mut food_intakes: Vec<FoodIntake> = vec![];

    for intake in iter {
        food_intakes.push(intake?);
    }

    Ok(food_intakes)
}