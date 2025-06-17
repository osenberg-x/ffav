use std::{error, fmt};

#[derive(Debug)]
pub enum DecodeError {
    UnsupportedFormat,
    InvalidArgument,
    InvalidData,
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DecodeError::UnsupportedFormat => write!(f, "Unsupported format"),
            DecodeError::InvalidArgument => write!(f, "Invalid argument"),
            DecodeError::InvalidData => write!(f, "Invalid data"),
        }
    }
}