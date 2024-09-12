use crate::matcher::Matcher;
use anyhow::{Context, Result};
pub use clap::Parser;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::{stdin, BufRead, BufReader};

#[cfg(test)]
mod tests {
    use super::{Cli, Parser};

    #[test]
    fn test_cli_parse() {
        match Cli::try_parse_from(vec!["minigrep", "-m", "10", "-i", "-x", "foo"]) {
            Ok(cli) => {
                assert_eq!(cli.pattern, "foo", "test#1");
                assert_eq!(cli.file_path, None, "test#2");
                assert_eq!(cli.max_lines, Some(10), "test#3");
                assert_eq!(cli.ignore_case, true, "test#4");
                assert_eq!(cli.basic_regex, false, "test#5");
                assert_eq!(cli.extended_regex, false, "test#6");
                assert_eq!(cli.invert_match, true, "test#7");
            }
            Err(e) => panic!("Failed to build CLI: {:?}", e),
        }

        match Cli::try_parse_from(vec!["minigrep", "-b", "-e", "foo"]) {
            Ok(cli) => panic!("Expected to fail, but got: {:?}", cli),
            Err(e) => assert!(
                e.to_string().contains(
                    "the argument '--basic-regex' cannot be used with '--extended-regex'"
                ),
                "test#8: {:?}",
                e
            ),
        }
    }
}

#[derive(Debug, Default, Parser)]
#[command(name = "minigrep")]
#[command(about = "A simple grep-like CLI tool", long_about = None)]
pub struct Cli {
    /// The pattern to search for
    pattern: String,
    /// The file path to read
    file_path: Option<String>,
    /// Maximum number of lines to read (optional, defaults to unlimited)
    #[arg(short, long, env("MAX_LINES"))]
    max_lines: Option<usize>,
    /// Perform a case-insensitive search (optional, defaults to false)
    #[arg(short, long, required(false), env("IGNORE_CASE"))]
    ignore_case: bool,
    /// Perform a basic regex search (optional, defaults to false)
    #[arg(
        short,
        long,
        required(false),
        env("BASIC_REGEX"),
        conflicts_with("extended_regex")
    )]
    basic_regex: bool,
    /// Perform an extended regex search (optional, defaults to false)
    #[arg(
        short,
        long,
        required(false),
        env("EXTENDED_REGEX"),
        conflicts_with("basic_regex")
    )]
    extended_regex: bool,
    /// Invert the search, i.e., show lines that do not match the pattern (optional, defaults to false)
    #[arg(short('x'), long, required(false), env("INVERT"))]
    invert_match: bool,
}

impl Cli {
    // Run the command
    // This method reads from the file or stdin and searches for the pattern.
    // It prints the matching lines to stdout.
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let reader: Box<dyn BufRead> = match &self.file_path {
            Some(path) => {
                // If file path is provided, read from the file
                let file = OpenOptions::new()
                    .read(true)
                    .open(path)
                    .with_context(|| format!("Failed to open file: {}", path))?;
                Box::new(BufReader::new(file))
            }
            None => {
                // If no file path is provided, read from stdin
                Box::new(BufReader::new(stdin()))
            }
        };

        // Configure the matcher
        let mut matcher = Matcher::new(&self.pattern);
        let matcher = matcher
            .with_max_lines(self.max_lines)
            .with_ignore_case(self.ignore_case)
            .with_basic_regex(self.basic_regex)
            .with_extended_regex(self.extended_regex)
            .with_invert_match(self.invert_match);

        // Search for the pattern and print the matching lines
        matcher
            .search(
                reader
                    .lines()
                    // Convert the error type to a Box<dyn Error>
                    .map(|line| line.map_err(|error| Box::new(error) as Box<dyn Error>)),
            )? // Propagate any errors
            .iter()
            .for_each(|line| println!("{}", line));

        Ok(())
    }
}
