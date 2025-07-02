use std::path::Path;
use ffav_utils::{ DataReader, FileReader };
use ffav_types::InputMetadata;
use crate::Demuxer;

// #[derive(Debug)]
pub struct DemuxerContext {
	demuxer: Option<Box<dyn Demuxer>>,
	input_metadata: Option<InputMetadata>,
	reader: Box<dyn DataReader>,
}

impl DemuxerContext {
	pub fn from_local(url: &str) -> Self {
		let file_reader = Box::new(FileReader::new(Path::new(url)).unwrap());

		Self {
			demuxer: None,
			input_metadata: None,
			reader: file_reader
		}
	}

	pub fn new(reader: Box<dyn DataReader>, input_metadata: Option<InputMetadata>) -> Self {
		Self {
			demuxer: None,
			input_metadata: input_metadata,
			reader: reader,
		}
	}

	pub fn get_reader_mut(&mut self) -> &mut Box<dyn DataReader> {
		&mut self.reader
	}

	pub fn get_input_metadata_mut(&mut self) -> &mut Option<InputMetadata> {
		&mut self.input_metadata
	}
}