use std::fmt;

#[derive(Copy, Clone)]
pub struct InitializationError;

impl fmt::Display for InitializationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Hermes initialization error")
    }
}
