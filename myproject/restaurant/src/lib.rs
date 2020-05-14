/*
 * mod front_of_house
{
    mod hosting
    {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving
    {
        fn take_order() {}

        fn server_order() {}

        fn take_payment() {}
    }
}
*/

use std::fmt;
use std::io;
use std::fmt::Result;
use std::io::Result as IoResult;
use std::op::Write;
use std::io::{self, Write};
use std::collections::*; //all pub

fn function1() -> fmt::Result
{}

fn function2() -> io::Result<()>
{}

fn function3() -> IoResult<()>
{}


mod front_of_house
{
    //mod hosting
    pub mod hosting
    {
        pub fn add_to_waitlist()
        {}
    }
}

/*
 * re-export
 */
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant()
{
    //absolute
    crate::front_of_house::hosting::add_to_waitlist();

    //relative
    front_of_house::hosting::add_to_waitlist();
}


fn serve_order()
{}

mod back_of_house
{
    pub struct Breakfast
    {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast
    {
        pub fn summer(toast: &str) -> Breakfast
        {
            Breakfast
            {
                totast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order()
    {
        cook_order();
        super::serve_order();
    }

    fn cook_order()
    {}
}

pub fn eat_at_restaurants()
{
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    //meal.seasonal_fruit = String::from("blueberries");
}
