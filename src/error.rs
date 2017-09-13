use std::fmt;
use std::error;

/// Error type for the crate.
///
/// This is shared across a lot of different modules within the crate.  The
/// description provided by the error, if not otherwise handled, will be seen by
/// the user of the program and thus should be clear enough and mostly
/// self-contained without requiring additional debug information.
#[derive(Debug)]
pub struct Error {
    description: String,
}

impl Error {
    pub fn new<S>(description: S) -> Self
    where
        S: Into<String>,
    {
        Error { description: description.into() }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(&self.description)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.description
    }
}
