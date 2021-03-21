use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct SimpleError {
    reason: String
}

impl SimpleError {
    pub fn new(reason: String) -> Self {
        Self {
            reason
        }
    }
}

impl fmt::Display for SimpleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.reason.as_str())
    }
}


impl Error for SimpleError { }