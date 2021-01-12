//! Error module
use std::error;
use std::fmt;
use std::io;

/// Options for possible errors that may arise
#[derive(Debug)]
pub enum Error {
    /// Standard I/O errors
    Io(io::Error),
    /// Parsing errors
    Parse(ParseError),
}

/// Enum for storing one of the possible errors code.
/// The associated value represents the row index where the error occurred.
#[derive(Debug)]
pub enum ParseError {
    /// Section has incorrect syntax
    IncorrectSection(usize),
    /// Unknown syntax format
    IncorrectSyntax(usize),
    /// Key has empty name
    EmptyKey(usize),
}

impl error::Error for Error {}
impl error::Error for ParseError {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(ref e) => e.fmt(f),
            Error::Parse(ref e) => e.fmt(f),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::IncorrectSection(line) => write!(f, "Incorrect section syntax at line {}", line),
            ParseError::IncorrectSyntax(line) => write!(f, "Incorrect syntax at line {}", line),
            ParseError::EmptyKey(line) => write!(f, "Key is empty at line {}", line),
        }
    }
}

impl From<ParseError> for Error {
    fn from(error: ParseError) -> Self {
        Error::Parse(error)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}
