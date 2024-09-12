use ::messenger::{Limiter, Messenger};
use std::cell::RefCell;

fn main() {
    let messenger = SilentMessenger::new();
    let mut limiter = Limiter::new(&messenger, 100);
    for i in (0..=100).step_by(10) {
        limiter.set_value(i);

        let got = messenger.sent();
        if got.is_empty() {
            continue;
        }

        println!("progress: {:3}%, messages sent:", i);
        for msg in got {
            println!(" -  {}", msg);
        }
    }
}

/// A Messenger instance that does not print messages to the console.
struct SilentMessenger {
    sent_messages: RefCell<Vec<String>>, // RefCell allows us to mutate the vector even though `self` is immutable
}

impl SilentMessenger {
    /// This function is used to create a new SilentMessenger instance.
    fn new() -> SilentMessenger {
        SilentMessenger {
            sent_messages: RefCell::new(vec![]),
        }
    }

    /// This function is used to get the messages sent by the messenger.
    fn sent(&self) -> Vec<String> {
        self.sent_messages.borrow().clone()
    }
}

impl Messenger for SilentMessenger {
    /// This function is used to send a message (implements the Messenger trait).
    fn send(&self, msg: &str) {
        // RefCell allows us to mutate the vector even though `self` is immutable
        self.sent_messages.borrow_mut().push(msg.to_string());
    }
}
