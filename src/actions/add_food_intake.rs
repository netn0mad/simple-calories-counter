use time::{OffsetDateTime};

use crate::{db::utils::get_formated_datetime};

pub struct FoodIntake;

impl super::action::Action for FoodIntake {
    const ACTION_KEY: &str = "add-intake";

    fn action() -> () {
        println!("Please, enter product data:");
        let name = super::input::get_string("Name".to_string());
        let calories = super::input::get_i32("Calories (per 100g)".to_string());
        let proteins = super::input::get_f32("Proteins (per 100g)".to_string());
        let fats = super::input::get_f32("Fats (per 100g)".to_string());
        let carbohydrates = super::input::get_f32("Carbohydrates (per 100g)".to_string());
        let weight = super::input::get_i32("Weight (per 100g)".to_string());
        let current_date_time;

        match OffsetDateTime::now_local() {
            Ok(local_time) => current_date_time = local_time,
            Err(_) => current_date_time = OffsetDateTime::now_utc(),
        }

        let formated_date_time = get_formated_datetime(current_date_time);

        let intake = crate::entities::food_intake::FoodIntake {
            product_name: name,
            calories: calories,
            proteins: proteins,
            fats: fats,
            carbohydrates: carbohydrates,
            weight: weight,
            eaten_at: formated_date_time
        };

        match crate::entities::food_intake::insert_food_intake(intake) {
            Ok(()) => (),
            Err(error) => println!("Error. {}", error),
        }
    }

    fn get_about_info() -> String {
        "'".to_string() + Self::ACTION_KEY + "' - this action allows you to add information about the product intake.\n\
        Specify its calories, proteins, fats and carbohydrates per 100 grams.\n\n"
    }
}

