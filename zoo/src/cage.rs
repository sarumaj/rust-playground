use crate::Animal;
use std::{fmt::Display, ops::Deref};

#[derive(Copy, Clone)]
pub struct Cage<'a, T: Animal>(&'a T);

impl<'a, T: Animal> Cage<'a, T> {
    pub fn new(animal: &'a T) -> Self {
        Cage(animal)
    }

    pub fn release(self) -> T {
        *self // Dereference the cage to get the animal
    }
}

impl<'a, T: Animal> Animal for Cage<'a, T> {
    fn noise(&self) -> String {
        self.0.noise()
    }

    fn pet() -> String {
        T::pet()
    }
}

impl<'a, T: Animal> Display for Cage<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Cage containing a {}", self.0)
    }
}

impl<'a, T: Animal> Deref for Cage<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
