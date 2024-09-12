use anyhow::{Context, Result};
use regex::{Regex, RegexBuilder};
use std::error::Error;

#[cfg(test)]
mod tests {
    use super::Matcher;

    #[test]
    fn test_matcher() {
        let matcher = Matcher::new("foo");
        assert_eq!(matcher.pattern, "foo");
        assert_eq!(matcher.ignore_case, false);
        assert_eq!(matcher.max_lines, None);
        assert_eq!(matcher.basic_regex, false);
        assert_eq!(matcher.extended_regex, false);
        assert_eq!(matcher.invert_match, false);

        let mut matcher = Matcher::new("foo");
        let matcher = matcher
            .with_basic_regex(true)
            .with_extended_regex(true)
            .with_ignore_case(true)
            .with_invert_match(true)
            .with_max_lines(Some(10));
        assert_eq!(matcher.pattern, "foo");
        assert_eq!(matcher.ignore_case, true);
        assert_eq!(matcher.max_lines, Some(10));
        assert_eq!(matcher.basic_regex, true);
        assert_eq!(matcher.extended_regex, true);
        assert_eq!(matcher.invert_match, true);
    }

    #[test]
    fn test_matcher_get_regex() {
        let matcher = Matcher::new("foo");
        let re = matcher.get_regex().unwrap();
        assert!(re.is_none(), "test#1");

        let mut matcher = Matcher::new("foo");
        let matcher = matcher.with_basic_regex(true);
        let re = matcher.get_regex().unwrap().unwrap();
        assert_eq!(re.as_str(), "foo", "test#2");

        let mut matcher = Matcher::new("foo");
        let matcher = matcher.with_extended_regex(true);
        let re = matcher.get_regex().unwrap().unwrap();
        assert_eq!(re.as_str(), "foo", "test#3");

        let mut matcher = Matcher::new("foo");
        let matcher = matcher.with_basic_regex(true).with_extended_regex(true);
        let re = matcher.get_regex().unwrap().unwrap();
        assert_eq!(re.as_str(), "foo", "test#4");

        let mut matcher = Matcher::new("foo");
        let matcher = matcher.with_ignore_case(true);
        let re = matcher.get_regex().unwrap();
        assert!(re.is_none(), "test#5");
    }

    #[test]
    fn test_matcher_get_match_fn() {
        let mut matcher = Matcher::new("foo");
        let matcher = matcher
            .with_basic_regex(true)
            .with_extended_regex(true)
            .with_ignore_case(true)
            .with_invert_match(true)
            .with_max_lines(Some(10));
        let match_fn = matcher.get_match_fn().unwrap();
        assert_eq!(match_fn("foo"), false, "test#1");
        assert_eq!(match_fn("bar"), true, "test#2");
        assert_eq!(match_fn("FOO"), false, "test#3");

        let matcher = matcher.with_invert_match(false);
        let match_fn = matcher.get_match_fn().unwrap();
        assert_eq!(match_fn("foo"), true, "test#4");
        assert_eq!(match_fn("bar"), false, "test#5");
        assert_eq!(match_fn("FOO"), true, "test#6");
    }

    #[test]
    fn test_matcher_search() {
        let mut matcher = Matcher::new("foo");
        let matcher = matcher
            .with_basic_regex(true)
            .with_extended_regex(true)
            .with_ignore_case(true)
            .with_invert_match(true)
            .with_max_lines(Some(10));
        let lines = vec!["foo", "bar", "FOO", "baz"];
        let matches = matcher
            .search(lines.into_iter().map(|line| Ok(line.to_string())))
            .unwrap();
        assert_eq!(matches, vec!["bar", "baz"], "test#1");

        let mut matcher = Matcher::new("foo");
        let matcher = matcher.with_invert_match(false);
        let lines = vec!["foo", "bar", "FOO", "baz"];
        let matches = matcher
            .search(lines.into_iter().map(|line| Ok(line.to_string())))
            .unwrap();
        assert_eq!(matches, vec!["foo"], "test#2");
    }
}

#[derive(Debug, Default)]
// The Matcher struct represents a pattern matcher that can search for lines that match a specified pattern.
pub struct Matcher {
    // The pattern field is a string that represents the pattern to search for.
    pub pattern: String,
    // The ignore_case field is a boolean flag that specifies whether to perform a case-insensitive search.
    pub ignore_case: bool,
    // The max_lines field is an optional usize value that specifies the maximum number of lines to read.
    pub max_lines: Option<usize>,
    // The basic_regex field is a boolean flag that specifies whether to use basic regular expressions.
    pub basic_regex: bool,
    // The extended_regex field is a boolean flag that specifies whether to use extended regular expressions.
    pub extended_regex: bool,
    // The invert_match field is a boolean flag that specifies whether to invert the match.
    pub invert_match: bool,
}

impl Matcher {
    // The new function is a constructor that creates a new Matcher instance with the specified pattern.
    // It takes a pattern string as an argument and returns a Matcher instance.
    // Other fields are set to their default values.
    pub fn new(pattern: &str) -> Self {
        Self {
            pattern: pattern.to_string(),
            ..Default::default()
        }
    }

    // This function returns a Result type, which is a common pattern in Rust.
    // The Result type is an enum that represents either success (Ok) or failure (Err).
    // The Ok variant contains the successful value, while the Err variant contains an error.
    // The Box<dyn Error> type is a trait object that can hold any type that implements the Error trait.
    // This allows the function to return any type of error.
    // The ? operator is used to propagate errors.
    // If the function returns an error, the ? operator will return early from the function and propagate the error to the caller.
    // This is a common pattern in Rust for handling errors.
    // The get_regex function returns an Option<Regex> type, which is an enum that represents either Some or None.
    // The Some variant contains the Regex value, while the None variant represents no value.
    // This allows the function to return either a valid regex or no regex.
    fn get_regex(&self) -> Result<Option<Regex>, Box<dyn Error>> {
        if !self.basic_regex && !self.extended_regex {
            return Ok(None); // Use default behavior
        }

        let mut builder = RegexBuilder::new(&self.pattern);
        if self.ignore_case {
            builder.case_insensitive(true);
        }

        if self.extended_regex {
            builder.ignore_whitespace(true);
        }

        let re = builder
            .build()
            .with_context(|| format!("Invalid regex pattern: {}", &self.pattern))?;

        Ok(Some(re))
    }

    // The get_match_fn function returns a Box<dyn Fn(&str) -> bool> type, which is a trait object
    // that represents a closure that takes a string slice and returns a boolean.
    // It calls the get_regex function to get the regex pattern, and then creates a closure that
    // checks if the line matches the regex pattern.
    // If no regex pattern is provided, it creates a closure that checks if the line contains the pattern.
    // This allows the function to return a closure that can be called with a string slice and return a boolean value.
    // The Box type is used to store the closure on the heap, which allows it to be returned from the function.
    // The Fn trait is a trait that represents a function or closure.
    // The &str type is a string slice, which is a reference to a string.
    // The -> bool type is the return type of the closure, which is a boolean value.
    // The Box<dyn Error> type is a trait object that can hold any type that implements the Error trait.
    // This allows the function to return any type of error.
    // The Ok(Box::new(...)) expression is used to create a new closure and return it as a trait object.
    // The move keyword is used to move the variables into the closure, which allows the closure to capture the variables by value.
    // This is necessary because the closure may outlive the variables, so they need to be moved into the closure to ensure they are valid.
    // The |line: &str| { ... } syntax is used to define the closure, which takes a string slice as an argument and returns a boolean value.
    // The let has_match = ...; expression is used to check if the line matches the regex pattern.
    // The invert && !has_match || !invert && has_match expression is used to invert the match if the invert_match flag is set.
    // The ? operator is used to propagate errors.
    fn get_match_fn(&self) -> Result<Box<dyn Fn(&str) -> bool>, Box<dyn Error>> {
        let re = self.get_regex()?;
        let invert = self.invert_match;
        match re {
            Some(re) => Ok(Box::new(move |line: &str| {
                let has_match = re.is_match(line);
                invert && !has_match || !invert && has_match
            })),
            None => {
                let pattern = self.pattern.clone();
                if self.ignore_case {
                    return Ok(Box::new(move |line: &str| {
                        let has_match = line.to_lowercase().contains(&pattern.to_lowercase());
                        invert && !has_match || !invert && has_match
                    }));
                }

                Ok(Box::new(move |line: &str| {
                    let has_match = line.contains(&pattern);
                    invert && !has_match || !invert && has_match
                }))
            }
        }
    }

    // The search function takes an iterator of lines and returns a Result<Vec<String>, Box<dyn Error>> type.
    // It uses the get_match_fn function to get the match function, and then iterates over the lines to find matches.
    // If the max_lines flag is set, it stops after reading the specified number of lines.
    // This allows the function to search for matches in a stream of lines and return the matching lines.
    // The I: Iterator<Item = Result<String, Box<dyn Error>>> type is a trait bound that specifies the type of the iterator.
    // The Item = Result<String, Box<dyn Error>> type specifies that the iterator yields a Result type with a string or an error.
    // The Ok(line) expression is used to extract the string from the Result type.
    // The match_fn(&line[..]) expression is used to call the match function with the line as an argument.
    // The matches.push(line) expression is used to add the line to the matches vector if it matches the pattern.
    // The line_count += 1 expression is used to increment the line count.
    // The if let Some(max_lines) = self.max_lines expression is used to check if the max_lines flag is set.
    // The break expression is used to stop reading lines if the line count exceeds the max_lines value.
    // The Ok(matches) expression is used to return the matching lines as a vector.
    // The ? operator is used to propagate errors.
    pub fn search<I>(&self, lines: I) -> Result<Vec<String>, Box<dyn Error>>
    where
        I: Iterator<Item = Result<String, Box<dyn Error>>>,
    {
        let match_fn = self.get_match_fn()?;
        let mut matches = Vec::new();
        let mut line_count = 0;

        for line in lines {
            let line = line?;
            if match_fn(&line[..]) {
                matches.push(line);
            }

            line_count += 1;
            if let Some(max_lines) = self.max_lines {
                if line_count >= max_lines {
                    break;
                }
            }
        }

        Ok(matches)
    }

    // The with_basic_regex function is a builder method that sets the basic_regex flag and returns a mutable reference to self.
    pub fn with_basic_regex(&mut self, basic_regex: bool) -> &mut Self {
        self.basic_regex = basic_regex;
        self
    }

    // The with_extended_regex function is a builder method that sets the extended_regex flag and returns a mutable reference to self.
    pub fn with_extended_regex(&mut self, extended_regex: bool) -> &mut Self {
        self.extended_regex = extended_regex;
        self
    }

    // The with_ignore_case function is a builder method that sets the ignore_case flag and returns a mutable reference to self.
    pub fn with_ignore_case(&mut self, ignore_case: bool) -> &mut Self {
        self.ignore_case = ignore_case;
        self
    }

    // The with_invert_match function is a builder method that sets the invert_match flag and returns a mutable reference to self.
    pub fn with_invert_match(&mut self, invert_match: bool) -> &mut Self {
        self.invert_match = invert_match;
        self
    }

    // The with_max_lines function is a builder method that sets the max_lines flag and returns a mutable reference to self.
    pub fn with_max_lines(&mut self, max_lines: Option<usize>) -> &mut Self {
        self.max_lines = max_lines;
        self
    }
}
