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

    // This is a closure that captures the variables a and b from the outer scope.
    // move keyword is used to transfer ownership of the variables a and b to the closure.
    return move || {
        let next: u128 = a;
        return match a.checked_add(b) {
            Some(sum) => {
                a = b;
                b = sum;
                Some(next)
            }
            None => None, // Return None if overflow occurs
        };
    };
}
