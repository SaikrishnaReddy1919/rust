use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    let secret = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("Please input your number : ");
        println!("Secret number is : {}", secret);
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read string");

        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        }; 
        println!("You guessed : {}", guess); 
        match guess.cmp(&secret) {
            Ordering :: Less => println!("{}","Too Small!".red()),
            Ordering :: Greater => println!("{}","Too Big!".red()),
            Ordering :: Equal => {
                println!("{}","You Win!".green());
                break;
            }
        }
    }
}
