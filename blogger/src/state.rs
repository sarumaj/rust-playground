use std::fmt::Display;

pub enum State {
    Draft,
    PendingReview,
    Published,
    Rejected,
}

impl State {
    pub fn new() -> Self {
        State::Draft
    }

    pub fn approve(self) -> State {
        match self {
            State::PendingReview => State::Published,
            _ => self,
        }
    }

    pub fn content<'a>(&self, post: &'a str) -> &'a str {
        match self {
            State::Published => post,
            _ => "",
        }
    }

    pub fn reject(self) -> State {
        match self {
            State::PendingReview => State::Rejected,
            _ => self,
        }
    }

    pub fn request_review(self) -> State {
        match self {
            State::Draft => State::PendingReview,
            State::Rejected => State::PendingReview,
            _ => self,
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            State::Draft => write!(f, "{:^16}", "Draft"),
            State::PendingReview => write!(f, "{:^16}", "Pending Review"),
            State::Published => write!(f, "{:^16}", "Published"),
            State::Rejected => write!(f, "{:^16}", "Rejected"),
        }
    }
}
