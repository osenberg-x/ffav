use std::path::Path;

use crate::DataKind;
use crate::DataReader;
use crate::DataReaderError;
use crate::FileReader;

pub struct SourceContext {
	pub data_kind: DataKind,
	pub reader: Option<Box<dyn DataReader>>,
}

impl SourceContext {
	pub fn new(data_kind: DataKind) -> Result<Self, DataReaderError> {
		let kind = data_kind.clone();
		let reader = match kind {
			DataKind::Local(url) => {
				let file_reader = FileReader::new(Path::new(url.as_str()))?;


				Some(Box::new(file_reader) as Box<dyn DataReader>)
			},
			DataKind::Memory(url) => {
				todo!()
			},
			DataKind::Network(url) => {
				todo!()
			},
			_ => {
				println!("Unknown data kind: {:?}", data_kind);
				None
			}
		};

		Ok(Self {
			data_kind: data_kind,
			reader,
		})
	}
}
