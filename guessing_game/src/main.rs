use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng() // returns ThreadRng
        .gen_range(1..101); // 1..101 or 1..=100 is a range from 1 to 100

    loop {
        println!("Please input your guess.");

        // Create mutable variable to store user input
        let mut guess = String::new();

        // Read user input and store it in the guess variable
        match std::io::stdin()// returns Stdin
            .read_line(&mut guess) // returns Result<usize, Error>
        { // or use expect(&str) to handle error (will panic if error)
            Ok(_) => {}, // Do nothing if the input is successful
            Err(_) => {
                println!("Failed to read line!");
                continue;
            }
        }

        // Convert the guess variable to a number
        let guess: u32 = match guess.trim()// returns &str
        .parse() // returns Result<F, F::Error>
        {
            Ok(num) => num,
            Err(_) => {
                // Check if the user wants to quit the game
                match guess.trim().to_lowercase().as_str() {
                    "quit" => {
                        println!("Quitting the game...");
                        break;
                    }
                    _ => {}
                }
                // If the user input is not a number, ask the user to enter a number
                println!("Please enter a number or \"quit\" to quit the game!");
                continue;
            }
        };

        // Use match to compare the guess with the secret number
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
