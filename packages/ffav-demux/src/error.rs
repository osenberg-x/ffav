use std::{error, fmt};

#[derive(Debug)]
pub enum DemuxError {
    UnsupportedFormat,
    InvalidArgument,
    InvalidData,
}

impl fmt::Display for DemuxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DemuxError::UnsupportedFormat => write!(f, "Unsupported format"),
            DemuxError::InvalidArgument => write!(f, "Invalid argument"),
            DemuxError::InvalidData => write!(f, "Invalid data"),
        }
    }
}