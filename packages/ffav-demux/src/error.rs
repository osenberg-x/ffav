use std::{error, fmt};

#[derive(Debug)]
pub enum DemuxError {
    OpenFileError,
    InvalidArgument,
    InvalidData,
}

impl fmt::Display for DemuxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DemuxError::OpenFileError => write!(f, "Open file error"),
            DemuxError::InvalidArgument => write!(f, "Invalid argument"),
            DemuxError::InvalidData => write!(f, "Invalid data"),
        }
    }
}