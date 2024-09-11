fn main() {
    println!("{}", convert_words("Hello, world!"));
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
