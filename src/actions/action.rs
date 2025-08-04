pub trait Action {
    const ACTION_KEY: &str;

    fn action() -> ();
    fn get_about_info() -> String;
}