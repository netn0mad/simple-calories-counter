pub struct AddProduct;

impl super::action::Action for AddProduct {
    const ACTION_KEY: &str = "add-product";

    fn action() -> () {
        println!("Hello, you run add_product Action! :)");
    }

    fn get_about_info() -> String {
        "This action makes it possible to add a product to the general\
        database for its further use.".to_string()
    }
}