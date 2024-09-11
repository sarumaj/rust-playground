#[cfg(test)]
mod tests {
    use super::fibonacci_generator;

    #[test]
    fn test_fibonacci_generator() {
        let mut fibonacci = fibonacci_generator();
        let mut previous: u128 = 0;
        let mut current: u128 = 1; // Start with the correct first two Fibonacci numbers

        for _ in 0..1000 {
            let result = fibonacci();
            match result {
                Some(value) => {
                    // Compare the generated Fibonacci value with the expected value
                    assert_eq!(
                        value, previous,
                        "Expected {:?} but got {:?}",
                        previous, value
                    );

                    // Compute the next Fibonacci numbers
                    let next = previous.checked_add(current);
                    previous = current;
                    current = match next {
                        Some(sum) => sum,
                        None => {
                            // If overflow would occur, check that the generator returns None
                            assert_eq!(fibonacci(), None, "Expected None but got Some(_)");
                            break;
                        }
                    };
                }
                None => {
                    // Ensure that once None is returned, the test is complete
                    break;
                }
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    let mut fibonacci = fibonacci_generator();

    let mut i: u32 = 0;
    loop {
        input.clear();

        println!("Press Enter to generate the next Fibonacci number or type 'exit' to quit!");

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input!");

        if input.trim() == "exit" {
            break;
        }

        match fibonacci() {
            Some(number) => println!("Fibonacci number no: {}, value: {}", i, number),
            None => {
                println!("Overflow occurred!");
                break;
            }
        }

        i += 1;
    }
}

// FnMut is a trait that defines a single method, call_mut, which is used to call the closure.
fn fibonacci_generator() -> impl FnMut() -> Option<u128> {
    let mut a: u128 = 0;
    let mut b: u128 = 1;
    let mut overflowed = false; // New variable to track if overflow has occurred

    return move || {
        if overflowed {
            return None; // If overflow has already occurred, return None
        }

        let next = a;
        match a.checked_add(b) {
            Some(sum) => {
                a = b;
                b = sum;
                Some(next)
            }
            None => {
                overflowed = true; // Set the overflow flag
                Some(next) // Return the last valid Fibonacci number
            }
        }
    };
}
