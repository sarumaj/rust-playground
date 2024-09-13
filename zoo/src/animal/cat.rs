use super::Animal;
use std::fmt::Display;

#[derive(Copy, Clone)]
pub struct Cat;

impl Animal for Cat {
    fn noise(&self) -> String {
        "Meow!".to_string()
    }

    fn pet() -> String {
        // spell-checker:ignore Purrrr
        "Purrrr".to_string()
    }
}

impl Display for Cat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Cat")
    }
}
