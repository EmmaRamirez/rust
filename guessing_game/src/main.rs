extern crate rand;

use std::io;
// io library for user input
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess your number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        // mutable variable declaration
        // :: <- associated function of String

        io::stdin().read_line(&mut guess) // pass mutable reference
            .expect("Failed to read line");
            // methods are only available on a
            // particular instance of the type

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // Use Rust's match and cmp to return Ordering enum
        match guess.cmp(&secret_number) {
            Ordering::Less      => println!("Too small!"),
            Ordering::Greater   => println!("Too big!"),
            Ordering::Equal     => {
                println!("You win!");
                break;
            }
        }
    }
}
