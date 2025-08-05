use crate::actions::action::Action;

pub mod action;
pub mod input;
pub mod add_food_intake;

pub fn get_full_actions_info() -> String {
    let mut info_accamulator = String::with_capacity(50);
    info_accamulator += "Action list: \n\n";
    info_accamulator += add_food_intake::FoodIntake::get_about_info().as_str();
    info_accamulator
}