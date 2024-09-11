use std::io::stdin;

#[cfg(test)]
mod tests {
    use super::{convert_word, convert_words};

    #[test]
    fn test_convert_word() {
        let helper = |word: &str, expected: &str| {
            let result = convert_word(word);
            assert_eq!(
                result, expected,
                "Expected '{}' but got '{}'",
                expected, result
            );
        };

        // spell-checker:ignore (words) ellohay apple orldway
        helper("hello", "ellohay");
        helper("apple", "apple");
        helper("world", "orldway");
    }

    #[test]
    fn test_convert_words() {
        // spell-checker:ignore (words) ellohay apple orldway
        assert_eq!(convert_words("hello apple world"), "ellohay apple orldway");
    }
}

fn main() {
    println!("Enter a sentence:");

    let mut sentence = String::new();
    stdin().read_line(&mut sentence).unwrap();

    println!("{}", convert_words(&sentence));
}

static VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn convert_word(word: &str) -> String {
    let mut chars = word
        .chars()
        .filter(|x| x.is_ascii_lowercase() || x.is_ascii_uppercase())
        .map(|x| x.to_ascii_lowercase())
        .into_iter();

    let first_char = chars.next().unwrap_or(' ').to_ascii_lowercase();
    if VOWELS.contains(&first_char) {
        return word.to_string();
    }

    let rest = chars.collect::<String>();
    format!("{}{}ay", rest, first_char)
}

fn convert_words(words: &str) -> String {
    let word_list = words
        .split_whitespace()
        .filter(|x| !x.is_empty())
        .collect::<Vec<&str>>();

    let mut result = String::new();
    for (index, word) in word_list.iter().enumerate() {
        result.push_str(&convert_word(word)[..]);
        if index == word_list.len() - 1 {
            break;
        }

        result.push(' ');
    }

    result
}
