use crate::DataKind;
use crate::DataReader;

pub struct SourceContext {
	pub data_kind: DataKind,
	pub reader: Option<Box<dyn DataReader>>,
}