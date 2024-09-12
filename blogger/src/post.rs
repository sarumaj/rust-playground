use crate::state::State;
use std::fmt::Display;

pub struct Post {
    state: Option<State>,
    content: String,
    prev: Option<Box<Post>>,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(State::new()),
            content: String::new(),
            prev: None,
        }
    }

    pub fn next(self) -> Post {
        match self.state {
            Some(State::Published) => {
                let mut next = Post::new();
                next.prev = Some(Box::new(self));
                next
            }
            _ => self,
        }
    }

    pub fn add_text(&mut self, text: &str) {
        match self.state {
            Some(State::Published) => return,
            _ => {
                self.content.push_str(text);
            }
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    pub fn content(&self) -> &str {
        self.state
            .as_ref()
            .unwrap_or(&State::new())
            .content(&self.content[..])
    }

    pub fn edit_text(&mut self, text: &str) {
        match self.state {
            Some(State::Published) | Some(State::PendingReview) => return,
            _ => {
                self.content = text.to_string();
            }
        }
    }

    pub fn prev(&self) -> Option<&Post> {
        self.prev.as_ref().map(|p| &**p)
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
}

impl Display for Post {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "({}) {}",
            self.state.as_ref().unwrap_or(&State::new()),
            self.content()
        )
    }
}
