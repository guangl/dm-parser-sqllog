use std::error::Error;
use std::fmt;
use std::num::{ParseFloatError, ParseIntError};

#[derive(Debug)]
pub enum ParseError {
    MissingFields(usize),
    Int(ParseIntError),
    Float(ParseFloatError),
    InvalidFormat,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::MissingFields(n) => write!(f, "missing fields: expected {} fields", n),
            ParseError::Int(e) => write!(f, "int parse error: {}", e),
            ParseError::Float(e) => write!(f, "float parse error: {}", e),
            ParseError::InvalidFormat => write!(f, "invalid format"),
        }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParseError::Int(e) => Some(e),
            ParseError::Float(e) => Some(e),
            _ => None,
        }
    }
}

impl From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> Self {
        ParseError::Int(e)
    }
}

impl From<ParseFloatError> for ParseError {
    fn from(e: ParseFloatError) -> Self {
        ParseError::Float(e)
    }
}
