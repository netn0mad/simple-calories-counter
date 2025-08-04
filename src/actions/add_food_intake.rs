pub struct FoodIntake;

impl super::action::Action for FoodIntake {
    const ACTION_KEY: &str = "add-intake";

    fn action() -> () {
        println!("Hello, you run add_intake Action! :)");
    }

    fn get_about_info() -> String {
        "'".to_string() + Self::ACTION_KEY + "' - this action allows you to add information about the product intake.\n\
        Specify its calories, proteins, fats and carbohydrates per 100 grams.\n\n"
    }
}