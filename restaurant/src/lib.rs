mod front_of_house;

// pub use let external code call hosting without having to make front_of_house public
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Shorter path thanks to use
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");

    // Change the kind of toast
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // Cannot change kind of fruit because it's private
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    // In a struct, members are private by default. We have to mention the pub keyword to make them public
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    
    // In an enum, every field is public if the enum is public
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

use std::fmt::Result;
use std::io::Result as IoResult;

// same as "use std::cmp::Ordering" and "use std::io"
use std::{cmp::Ordering, io};

// same as "use std::io" and "use std::io::Write"
// use std::io::{self, Write};

// use carefully
use std::collections::*;