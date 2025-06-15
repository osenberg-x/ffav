use std::{error, fmt};

#[derive(Debug)]
pub enum CodecError {
    UnsupportedFormat,
    InvalidArgument,
    InvalidData,
}

impl fmt::Display for CodecError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CodecError::UnsupportedFormat => write!(f, "Unsupported format"),
            CodecError::InvalidArgument => write!(f, "Invalid argument"),
            CodecError::InvalidData => write!(f, "Invalid data"),
        }
    }
}