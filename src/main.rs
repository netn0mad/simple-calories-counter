use clap::Parser;
use crate::actions::action::Action;

mod entities;
mod actions;
mod db;

const COMMAND_ABOUT: &str = "A simple console utility for counting calories.\
It can store and display various statistics and information about products, calories, etc.
that you have previously recorded.";

const DB_NAME: &str = "calories.db";

#[derive(Parser, Debug)]
#[command(version, about = COMMAND_ABOUT, long_about = None)]
struct Args {
    #[arg(short = 'a', long = "action", required = false)]
    action: Option<String>,

    #[arg(short = 'l', long = "action-list", required = false)]
    action_list: bool,
}

fn main() {
    let args = Args::parse();

    if args.action_list {
        println!("{}", actions::get_full_actions_info());
        return;
    }

    match db::init::init() {
        Ok(_) => (),
        Err(exception) => println!("Error database initialization: {}", exception),
    };

    if let Some(action) = args.action {
        match action.as_str() {
            actions::add_food_intake::FoodIntake::ACTION_KEY => {
                actions::add_food_intake::FoodIntake::action();
            },
            _ => println!("Action not found!"),
        };
    } else {
        println!("Please specify an action. Use --action-list to see available actions. Or use --help.");
    }
}