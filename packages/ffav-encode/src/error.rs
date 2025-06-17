use std::{error, fmt};

#[derive(Debug)]
pub enum EncodeError {
    UnsupportedFormat,
    InvalidArgument,
    InvalidData,
}

impl fmt::Display for EncodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EncodeError::UnsupportedFormat => write!(f, "Unsupported format"),
            EncodeError::InvalidArgument => write!(f, "Invalid argument"),
            EncodeError::InvalidData => write!(f, "Invalid data"),
        }
    }
}