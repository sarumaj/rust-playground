mod cat;
mod dog;

pub use cat::Cat;
pub use dog::Dog;

use std::fmt::Display;

pub trait Animal: Display + Copy {
    fn noise(&self) -> String;

    fn pet() -> String;
}
