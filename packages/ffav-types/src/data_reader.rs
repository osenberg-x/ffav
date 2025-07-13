use arc_slice::layout::{BoxedSliceLayout};
use arc_slice::{ArcBytes};
use crate::DataChunk;
use std::path::Path;
use std::{
    fs::File,
    io::{Error, ErrorKind, Read, Seek, SeekFrom},
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
    fn read_chunk(&mut self, size: u64) -> Result<DataChunk, DataReaderError>;
    fn seek(&mut self, offset: i64) -> Result<u64, DataReaderError>;

    fn close(&mut self) -> Result<(), DataReaderError>;
}

#[derive(Debug)]
pub struct FileReader {
    file: Option<File>,
    position: u64,
}

impl FileReader {
    pub fn new(file_p: &Path) -> Result<Self, DataReaderError> {
        let file = File::open(file_p)?;
        Ok(Self {
            file: Some(file),
            position: 0,
        })
    }
}

impl DataReader for FileReader {
    /// Reads a data chunk of specified size from the current file position.
    ///
    /// # Parameters
    /// - `size`: Number of bytes to read (must be > 0)
    ///
    /// # Returns
    /// - `Ok(Some(DataChunk))`: Successfully read a full chunk of data
    /// - `Ok(None)`: Reached EOF before reading a complete chunk
    /// - `Err(DataReaderError::IO(...))`: I/O operation failed
    ///
    /// # Behavior
    /// - Updates internal position only on successful reads
    /// - Returns None when EOF is encountered before completing the read
    fn read_chunk(&mut self, size: u64) -> Result<DataChunk, DataReaderError> {
        let file = self.file.as_mut().ok_or(DataReaderError::IO(Error::new(
            ErrorKind::Other,
            "file not open",
        )))?;

        let mut buffer = vec![0u8; size as usize];
        match file.read_exact(&mut buffer) {
            Ok(()) => {
                self.position += size;
                let slice_buffer = ArcBytes::<BoxedSliceLayout>::from(buffer);
                let data_chunk = DataChunk { data: slice_buffer };
                Ok(data_chunk)
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                let actual_size = buffer.len()
                    - e.get_ref()
                        .map(|e| {
                            e.downcast_ref::<std::io::Error>()
                                .map(|e| e.raw_os_error())
                                .flatten()
                                .unwrap_or(0) as usize
                        })
                        .unwrap_or(0);

                if actual_size > 0 {
                    self.position += actual_size as u64;
                    let slice_buffer = ArcBytes::<BoxedSliceLayout>::from(buffer);
                    let data_chunk = DataChunk { data: slice_buffer };
                    Ok(data_chunk)
                } else {
                    Err(DataReaderError::IO(std::io::Error::new(
                        ErrorKind::UnexpectedEof,
                        "unexpected eof",
                    )))
                }
            }
            Err(e) => Err(DataReaderError::IO(e)),
        }
    }

    /// Moves the file pointer by `offset` bytes relative to the current position.
    ///
    /// # Parameters
    /// - `offset`: The number of bytes to move the file pointer (positive values move forward,
    ///             negative values move backward).
    ///
    /// # Returns
    /// - `Ok(u64)`: The new file pointer position in bytes.
    /// - `Err(...)`:
    ///   - `DataReaderError::IO(...)`: If an I/O operation fails.
    fn seek(&mut self, offset: i64) -> Result<u64, DataReaderError> {
        let file = self.file.as_mut().ok_or(DataReaderError::IO(Error::new(
            ErrorKind::Other,
            "file not open",
        )))?;

        let new_pos = file.seek(SeekFrom::Current(offset))?;
        self.position = new_pos;
        Ok(new_pos)
    }

    /// Releases the file resource by taking ownership of the inner `File` and dropping it.
    ///
    /// This function consumes the internal `File` handle (if any) by calling `Option::take()`,
    /// which transfers ownership and sets the internal state to `None`. The file will be
    /// automatically closed when the returned `File` value is dropped.
    ///
    /// **Note:** This implementation does *not* explicitly call `sync_all()` or handle any
    /// I/O errors that might occur during file closure or synchronization. If guaranteed
    /// data persistence is required, consider adding explicit synchronization logic.
    ///
    /// # Returns
    /// - `Ok(())`: Always returns success, as no error handling is implemented for file closure.
    ///
    /// # Example
    /// ```rust
    /// reader.close()?; // Releases the file resource
    /// ```
    fn close(&mut self) -> Result<(), DataReaderError> {
        self.file.take();
        Ok(())
    }
}
