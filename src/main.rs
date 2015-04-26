extern crate rand;
use std::io;
use std::cmp::Ordering;
use std::cmp::Ordering::*;
use std::result::Result;
use std::result::Result::*;
use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    println!("Guess the number!");

    let secret_number = (rand::random::<u32>() % 100) + 1;

    loop {
        println!("Please input your guess.");

        let mut input = "".to_string();
        io::stdin().read_line(&mut input)
                           .ok()
                           .expect("Failed to read line");

        let input_num: Result<u32, ParseIntError> = FromStr::from_str(&input.trim());
        let num = match input_num {
            Ok(num) => num,
            _     => {
                println!("Please input a number!");
                continue;
            }
        };

        println!("You guessed: {}", num);

        match cmp(num, secret_number) {
            Less    => println!("Too small!"),
            Greater => println!("Too big!"),
            Equal   => {
                println!("You win!");
                return;
            },
        }
    }
}

fn cmp(a: u32, b: u32) -> Ordering {
    if a < b { Less }
    else if a > b { Greater }
    else { Equal }
}
