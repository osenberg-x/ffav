use crate::DataKind;
use crate::DataWriter;

pub struct SinkContext {
	pub data_kind: DataKind,
	pub writer: Option<Box<dyn DataWriter>>,
}