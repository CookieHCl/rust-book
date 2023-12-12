mod front_of_house;

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    // each field is public or not in struct
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // public field in tuple struct
    pub struct Point(pub i32, pub i32);

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // all variants are public in enum
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // use
    use crate::front_of_house::hosting;
    hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

// as keyword
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    Result::Ok(())
}

fn function2() -> IoResult<()> {
    IoResult::Ok(())
}

// nested path
use std::{cmp::Ordering, collections};
// equivalent to
// use std::cmp::Ordering;
// use std::collections;

use std::io::{self, Write};
// equivalent to
// use std::io;
// use std::io::Write;

// glob operator
use std::collections::*;
