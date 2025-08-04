pub struct AddProduct;

impl super::action::Action for AddProduct {
    const ACTION_KEY: &str = "add-product";

    fn action() -> () {
        println!("Hello, you run add_product Action! :)");
    }

    fn get_about_info() -> String {
        "'".to_string() + Self::ACTION_KEY + "' - this action makes it possible to add a product to the general\n\
        database for its further use.\n\n"
    }
}