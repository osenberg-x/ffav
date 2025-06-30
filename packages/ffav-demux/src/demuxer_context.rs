use ffav_utils::DataReader;
use ffav_types::InputFormat;

// #[derive(Debug)]
pub struct DemuxerContext {
	input_format: Option<InputFormat>,
	pub reader: Box<dyn DataReader>,
}

impl DemuxerContext {
	pub fn new(reader: Box<dyn DataReader>, input_format: Option<InputFormat>) -> Self {
		Self {
			reader,
			input_format: input_format,
		}
	}

	pub fn get_reader(&mut self) -> &Box<dyn DataReader> {
		&self.reader
	}

	pub fn get_input_format(&self) -> Option<&InputFormat> {
		self.input_format.as_ref()
	}
}