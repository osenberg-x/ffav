use std::{error, fmt};

#[derive(Debug)]
pub enum MuxError {
    UnsupportedFormat,
    InvalidArgument,
    InvalidData,
}

impl fmt::Display for MuxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MuxError::UnsupportedFormat => write!(f, "Unsupported format"),
            MuxError::InvalidArgument => write!(f, "Invalid argument"),
            MuxError::InvalidData => write!(f, "Invalid data"),
        }
    }
}