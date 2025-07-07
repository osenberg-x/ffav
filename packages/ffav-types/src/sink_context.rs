use crate::DataKind;
use crate::DataWriter;

pub struct SinkContext {
	pub data_kind: DataKind,
	pub writer: Option<Box<dyn DataWriter>>,
}

impl SinkContext {
	pub fn new(data_kind: DataKind, writer: Option<Box<dyn DataWriter>>) -> Self {
		Self {
			data_kind,
			writer,
		}
	}
}
