use crate::{db::utils::{get_current_datetime}, entities::food_intake::FoodIntake};

pub struct InfoToday;

impl super::action::Action for InfoToday {
    const ACTION_KEY: &str = "info-today";

    fn action() -> () {
        let intakes = crate::entities::food_intake::get_data_for_day(get_current_datetime()).expect("Get intakes error.");

        let mut summary_food_intake = FoodIntake::new_empty();

        for intake in intakes.clone() {
            summary_food_intake = summary_food_intake + intake;
        }

        println!("--------------------------------------");
        println!("-- Данные на {} --------------", summary_food_intake.eaten_at);
        println!("--------------------------------------");

        println!("-- Калорий:   {:.2}", summary_food_intake.calories);
        println!("-- Белка:     {:.2}", summary_food_intake.proteins);
        println!("-- Жиров:     {:.2}", summary_food_intake.fats);
        println!("-- Углеводов: {:.2}", summary_food_intake.carbohydrates);
        println!("-- Вес:       {:.2}", summary_food_intake.weight);

        println!("----------------------------------------");

        for intake in intakes {
            todo!("Continue...");
        }
    }

    fn get_about_info() -> String {
        "'".to_string() + Self::ACTION_KEY + "' - This action allows you to get information about today's calorie consumption.\n\n"
    }
}
