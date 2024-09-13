use super::Animal;
use std::fmt::Display;

#[derive(Copy, Clone)]
pub struct Dog;

impl Animal for Dog {
    fn noise(&self) -> String {
        "Woof!".to_string()
    }

    fn pet() -> String {
        // spell-checker:ignore Grrrrr
        "Grrrrr".to_string()
    }
}

impl Display for Dog {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Dog")
    }
}
