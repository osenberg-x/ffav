use arc_slice::layout::{ArcLayout, BoxedSliceLayout, DefaultLayout, VecLayout};
use arc_slice::{ArcBytes, ArcBytesMut};
use ffav_types::DataChunk;
use ffav_types::DataKind;
use std::path::Path;
use std::{
    fs::File,
    io::{BufReader, Read},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataReaderError {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Network error: {0}")]
    Network(String),
}

pub trait DataReader: Send + Sync {
    fn read_chunk(&mut self, size: usize) -> Result<Option<DataChunk>, DataReaderError>;
    fn seek(&mut self, position: u64) -> Result<(), DataReaderError>;

    fn close(&mut self) -> Result<(), DataReaderError>;
}

#[derive(Debug)]
pub struct FileReader {
    handle: File,
    position: usize,
}

impl FileReader {
    fn new(file_p: &Path) -> Result<Self, DataReaderError> {
        let handle = File::open(file_p)?;
        Ok(Self {
            handle,
            position: 0,
        })
    }
}

impl DataReader for FileReader {
    fn read_chunk(&mut self, size: usize) -> Result<Option<DataChunk>, DataReaderError> {
        let mut buffer = vec![0u8; size];
        self.handle.read_exact(&mut buffer).unwrap();

        let slice_buffer = ArcBytes::<BoxedSliceLayout>::from(buffer);

        let data_chunk = DataChunk { data: slice_buffer };

        self.position += size;

        Ok(Some(data_chunk))
    }

    fn seek(&mut self, position: u64) -> Result<(), DataReaderError> {
        todo!()
    }

    fn close(&mut self) -> Result<(), DataReaderError> {
        todo!()
    }
}
