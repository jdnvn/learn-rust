mod front_of_house;
mod back_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    front_of_house::serving::take_order();
    let mut meal = back_of_house::Breakfast::summer("White");
    meal.toast = String::from("Rye");
    back_of_house::cook_order();
    front_of_house::serving::serve_order();
    front_of_house::serving::take_payment();
}
