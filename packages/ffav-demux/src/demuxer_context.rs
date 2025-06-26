use ffav_utils::DataReader;

// #[derive(Debug)]
pub struct DemuxerContext {
	pub reader: Box<dyn DataReader>,
}

impl DemuxerContext {
	pub fn new(reader: Box<dyn DataReader>) -> Self {
		Self {
			reader,
		}
	}

	pub fn get_reader(&mut self) -> &Box<dyn DataReader> {
		&self.reader
	}
}