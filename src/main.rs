use std::io::{self, stdout, BufWriter};
use rand::Rng;
use std::cmp::Ordering;
use ferris_says::say;
use colored::*;

extern crate ferris_says;
fn main() {
    println!("Guess the secret number from 1 - 100.");

    let secret_number = rand::thread_rng().gen_range(1..101);

    let out = format!("You got it! The secret number is: {}", secret_number);

    let width = 90;

    let mut writer = BufWriter::new(stdout());

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too low!".red()),
            Ordering::Greater => println!("{}", "Too high!".red()),
            Ordering::Equal => {
                say(out.as_bytes(), width, &mut writer).unwrap();
                break;
            }
        }
    }
}
