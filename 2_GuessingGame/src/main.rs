use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=100);

    // Loop is just an infinite loop
    loop {
        println!("Please input your guess:");

        // Vars in rust are immutable by default
        let mut guess = String::new();

        // Argument must be mutable so it can be overwritten,
        // passed by reference
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Shadow guess with an u32 instead of making a new var.
        // From this, Rust infers secret_number should be u32 instead
        // of the default i32 as well
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number!");
                continue;
            }
        };

        println!("You guessed: {guess}");

        // Pattern is Ordering returned by cmp,
        // compare to the three arms
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
            Ordering::Greater => println!("Too big!"),
        }
    }
}
