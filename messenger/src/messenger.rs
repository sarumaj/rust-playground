/// A trait for sending messages
pub trait Messenger {
    /// This function is used to send a message.
    fn send(&self, msg: &str);
}
