use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct UltraNLPError {
    message: String,
}

impl UltraNLPError {
    pub fn new<T: AsRef<str>>(message: T) -> Self {
        let message = message
            .as_ref()
            .to_string();

        Self { message }
    }
}

impl fmt::Display for UltraNLPError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for UltraNLPError {}

pub type UltraNLPResult<T, E = UltraNLPError> = Result<T, E>;
