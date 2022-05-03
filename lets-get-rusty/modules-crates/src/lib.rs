// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// pub fn eat_at_restaurant() {
//     // Absolute path
//     crate::front_of_house::hosting::add_to_waitlist();

//     // Relative path
//     front_of_house::hosting::add_to_waitlist();
// }

//----A-START---------above code works fine, below code is demo to separate above code into modules, and load from them--------//


// Using a semicolon after mod front_of_house rather than using a block tells Rust to load 
//the contents of the module from another file with the same name as the module.
mod front_of_house; //loads front_of_house.rs 1 (check front_of_hosue.rs for 2)

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
pub fn check () {}


//------A-END--till here module separation goes-------------//

//-----------------------------------Starting Relative Paths with super-----------------------------//
//The fix_incorrect_order function is in the back_of_house module, so we can use super
//to go to the parent module of back_of_house, which in this case is crate, the root.
// From there, we look for serve_order and find it. Success!
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}
//-----------------------------------Making Structs-----------------------------//
//note that because back_of_house::Breakfast has a private field, the struct needs to provide
//a public associated function that constructs an instance of Breakfast (we’ve named it summer here).
// If Breakfast didn’t have such a function, we couldn’t create an instance of Breakfast in
//eat_at_restaurant because we couldn’t set the value of the private seasonal_fruit field in eat_at_restaurant.
mod back_of_house1 {
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
}

pub fn eat_at_restaurant1() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house1::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}

//-----------------------------------Enums Public-----------------------------//

//In contrast, if we make an enum public, all of its variants are then public.
//We only need the pub before the enum keyword.
mod back_of_house2 {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant2() {
    let order1 = back_of_house2::Appetizer::Soup;
    let order2 = back_of_house2::Appetizer::Salad;
}

//--------------------------Bringing Paths into Scope with the use Keyword-----------------------//
mod front_of_house_a {
    pub mod hosting_a {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house_a::hosting_a;
//or replace above line with
//use self::front_of_house::hosting;
pub fn eat_at_restaurant_a() {
    hosting_a::add_to_waitlist();
    hosting_a::add_to_waitlist();
    hosting_a::add_to_waitlist();
}

//--------------------------Creating Idiomatic use Paths-----------------------//
mod front_of_house_b {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house_b::hosting::add_to_waitlist;

pub fn eat_at_restaurant_b() {
    //if we have multiple same function as 'add_to_waitlist' then we cant figure out to which module
    // the following functions are belong to. So, it’s idiomatic to specify the full path
    add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();
}

//--------------------------Creating correct Paths-----------------------//
use std::fmt;
use std::io;


//or

// use std::fmt::Result;
// use std::io::Result as IoResult;


//below Result belongd to fmt
fn function1() -> fmt::Result {
    // --snip--
    Ok(())
}

//below Result belongd to io
fn function2() -> io::Result<()> {
    // --snip--
    Ok(())
}

//--------------------------Re-exporting Names with pub use-----------------------//
//This technique is called re-exporting because we’re bringing an item into scope but also 
//making that item available for others to bring into their scope.
mod front_of_house_c {
    pub mod hosting_c {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house_c::hosting_c;

pub fn eat_at_restaurant_c() {
    hosting_c::add_to_waitlist();
    hosting_c::add_to_waitlist();
    hosting_c::add_to_waitlist();
}


//--------TIPS--------//

// use std::cmp::Ordering;
// use std::io;
//or
//use std::{cmp::Ordering, io};

//--
// use std::io;
// use std::io::Write;
//or
// use std::io::{self, Write};

//--
// use std::collections::*;
