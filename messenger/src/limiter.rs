use crate::messenger::Messenger;

/// A Limiter instance is used to limit the value of a counter and send a message when the value exceeds a certain threshold.
pub struct Limiter<'t, T: Messenger> {
    /// The Messenger instance to use for sending messages.
    messenger: &'t T, // The lifetime of the Limiter instance is tied to the lifetime of the Messenger instance
    /// The maximum value the counter can have.
    max: u32,
    /// The current value of the counter.
    value: u32,
}

impl<'t, T> Limiter<'t, T>
where
    T: Messenger,
{
    /// This function is used to create a new Limiter instance with the given Messenger instance and maximum value.
    pub fn new(messenger: &'t T, max: u32) -> Limiter<'t, T> {
        Limiter {
            messenger,
            max,
            value: 0,
        }
    }

    /// This function is used to set the value of the counter.
    /// It takes the new value of the counter and sends a message if the value exceeds a certain threshold.
    /// The threshold is 100%, 90%, and 75% of the maximum value.
    pub fn set_value(&mut self, value: u32) {
        self.value = value;

        let percent = self.value as f64 / self.max as f64 * 100.0;

        let message = if percent >= 100.0 {
            Some("Warning: You have exceeded your quota")
        } else if percent >= 90.0 {
            Some("Urgent warning: You have exceeded 90% of your quota")
        } else if percent >= 75.0 {
            Some("Warning: You have exceeded 75% of your quota")
        } else {
            None
        };

        if let Some(msg) = message {
            self.messenger.send(msg);
        }
    }
}
