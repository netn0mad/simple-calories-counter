use crate::entities::product::Product;

pub struct FoodIntake;

impl super::action::Action for FoodIntake {
    const ACTION_KEY: &str = "add-intake";

    fn action() -> () {
        println!("Please, enter product data:");
        let name = super::input::get_string("Name".to_string());
        let calories = super::input::get_f32("Calories (per 100g)".to_string());
        let proteins = super::input::get_f32("Proteins (per 100g)".to_string());
        let fats = super::input::get_f32("Fats (per 100g)".to_string());
        let carbohydrates = super::input::get_f32("Carbohydrates (per 100g)".to_string());
        let weight = super::input::get_i32("Weight".to_string());
        

        let intake = crate::entities::food_intake::FoodIntake::new(Product {
                name,
                calories,
                proteins,
                fats,
                carbohydrates,
            },
            weight);

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

