fn main() {
    let mut s1 = no_dangle();

    let len = calculate_length(&s1); // prevents s1 from going out of scope here

    println!("The length of '{}' is {}.", s1, len);

    change(&mut s1);
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
