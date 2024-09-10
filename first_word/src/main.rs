fn main() {
    let mut my_string = String::from("hello world");

    println!("{}", first_word(&my_string));
    println!("{}", last_word(&my_string));

    my_string.clear();
}

// we pass a reference to the string to the first_word function
// so that we don't take ownership of the string
// we use &str instead of &String because we don't need all the methods
// that String has
// &str is a slice of the string
// slices are references to a part of a string
// slices have a type &[T]
// we return a slice of the string
fn x_word(s: &str, last: bool) -> &str {
    let s = s.trim(); // Remove leading and trailing whitespace
    if last {
        // Find the last occurrence of a space
        return match s.rfind(' ') {
            Some(index) => &s[index + 1..], // Return slice up to the last space
            None => s,                      // No space found; return the entire string
        };
    } else {
        // Find the first occurrence of a space
        return match s.find(' ') {
            Some(index) => &s[..index], // Return slice up to the first space
            None => s,                  // No space found; return the entire string
        };
    }
}

fn first_word(s: &str) -> &str {
    return x_word(s, false);
}

fn last_word(s: &str) -> &str {
    return x_word(s, true);
}
