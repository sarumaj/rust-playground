fn main() {
    print_labeled_measurement(5, 'h');

    // Expressions
    let y = {
        let x = 3;
        let x = plus_one(x); // shadowed
        x + 1 // no semicolon for return values
    };

    print_labeled_measurement(y, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn plus_one(x: i32) -> i32 {
    x + 1 // no semicolon for return values
}
