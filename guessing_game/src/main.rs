use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please, input your guess.");

        let mut guess = String::new(); // mutable variable

        io::stdin().
            read_line(&mut guess). // pass reference (prevents passing a copy)
            expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => print!("Too small!"),
            Ordering::Greater => print!("Too big!"),
            Ordering::Equal => {
                print!("You win!");
                break;
            }
        };
    }
}
