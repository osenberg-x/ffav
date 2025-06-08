use std::{error, fmt};

#[derive(Debug)]
pub enum FormatError {
    UnsupportedFormat,
    InvalidArgument,
    InvalidData,
}

impl fmt::Display for FormatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FormatError::UnsupportedFormat => write!(f, "Unsupported format"),
            FormatError::InvalidArgument => write!(f, "Invalid argument"),
            FormatError::InvalidData => write!(f, "Invalid data"),
        }
    }
}