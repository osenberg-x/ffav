use ffav_types::DataReaderError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DemuxError {
    #[error("Argument error")]
    ArgumentError,
    #[error("No demuxer found")]
    NoDemuxerFound,
    #[error("Reader error: {0}")]
    ReaderError(#[from] DataReaderError),
}
