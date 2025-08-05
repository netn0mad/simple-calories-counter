use std::io::{self, Write};
use time::{OffsetDateTime};

use crate::{db::utils::get_formated_datetime};

pub struct FoodIntake;

impl super::action::Action for FoodIntake {
    const ACTION_KEY: &str = "add-intake";

    fn action() -> () {
        println!("Please, enter product data:");
        let product_name = get_product_name();
        let product_weight = get_product_weight();
        let current_date_time;

        match OffsetDateTime::now_local() {
            Ok(local_time) => current_date_time = local_time,
            Err(_) => current_date_time = OffsetDateTime::now_utc(),
        }

        let formated_date_time = get_formated_datetime(current_date_time);

        match crate::entities::food_intake::insert_food_intake(product_name, formated_date_time, product_weight) {
            Ok(()) => (),
            Err(error) => println!("Error. {}", error),
        }
    }

    fn get_about_info() -> String {
        "'".to_string() + Self::ACTION_KEY + "' - this action allows you to add information about the product intake.\n\
        Specify its calories, proteins, fats and carbohydrates per 100 grams.\n\n"
    }
}

fn get_product_name() -> String {
    loop {
        print!("Name: ");
        io::stdout().flush().expect("Error buffer flush.");

        let mut product_name = String::new();

        io::stdin()
            .read_line(&mut product_name)
            .expect("Error. Bad input");

        let product_name = product_name.trim().to_string();

        if product_name.is_empty() {
            println!("Please enter a non-empty string");
            continue;
        }
        
        return product_name;
    }
}

fn get_product_weight() -> i32 {
    loop {
        print!("Weight (integer): ");
        io::stdout().flush().expect("Error buffer flush.");

        let mut product_weight = String::new();

        io::stdin()
            .read_line(&mut product_weight)
            .expect("Error. Bad input");

        return match product_weight.trim().parse::<i32>() {
            Ok(weight) => weight,
            Err(_) => 0, 
        };
    }
}