use std::{error, fmt};

/// Error type for all fallible operations in this crate.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Error {
    OutOfBounds,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::OutOfBounds => write!(f, "out of bounds"),
        }
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }
}
