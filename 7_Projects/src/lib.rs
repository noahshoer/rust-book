mod back_of_house;

pub use crate::back_of_house::Breakfast as Brekky;
pub use back_of_house::Toast;

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast.
    let mut meal = Brekky::summer(Toast::Rye);
    // Change our mind about what bread we'd like.
    meal.toast = Toast::Wheat;
    println!("I'd like {:?} toast please", meal.toast);

    meal.eat();

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal.
    // meal.seasonal_fruit = String::from("blueberries");
}