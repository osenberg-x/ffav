use crate::DataChunk;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataWriterError {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Network error: {0}")]
    Network(String),
}

pub trait DataWriter: Send + Sync {
    fn write_chunk(&mut self, chunk: &DataChunk) -> Result<(), DataWriterError>;
    fn flush(&mut self) -> Result<(), DataWriterError>;
    fn close(&mut self) -> Result<(), DataWriterError>;
}