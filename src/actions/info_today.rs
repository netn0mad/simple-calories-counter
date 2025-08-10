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

        Self.print_intake_data(summary_food_intake);

        println!("----------------------------------------");
        println!("-- Детальная информация");

        for intake in intakes {
            println!("----------------------------------------");
            Self.print_intake_data(intake);
            println!("----------------------------------------");
        }
    }

    fn get_about_info() -> String {
        "'".to_string() + Self::ACTION_KEY + "' - This action allows you to get information about today's calorie consumption.\n\n"
    }
}

impl InfoToday {
    fn print_intake_data(&self, intake: FoodIntake) {
        if !intake.product_name.is_empty() {
            println!("-- Название:  {}", intake.product_name);
        }
        
        println!("-- Калорий:   {:.2}", intake.calories);
        println!("-- Белка:     {:.2}", intake.proteins);
        println!("-- Жиров:     {:.2}", intake.fats);
        println!("-- Углеводов: {:.2}", intake.carbohydrates);
        println!("-- Вес:       {:.2}", intake.weight);
    }
}
