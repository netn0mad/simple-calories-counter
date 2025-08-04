use clap::Parser;
use crate::actions::action::Action;

mod actions;

const COMMAND_ABOUT: &str = "A simple console utility for counting calories.\
It can store and display various statistics and information about products, calories, etc.
that you have previously recorded.";

#[derive(Parser, Debug)]
#[command(version, about = COMMAND_ABOUT, long_about = None)]
struct Args {
    #[arg(short, long)]
    action: String,
}

fn main() {
    let args = Args::parse();

    match args.action.as_str() {
        actions::add_product::AddProduct::ACTION_KEY => {
            actions::add_product::AddProduct::action();
        },
        _ => println!("Action not found!"),
    };
}