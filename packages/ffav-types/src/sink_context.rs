use crate::{DataKind, DataWriter, DataWriterError};

pub struct SinkContext {
	pub data_kind: DataKind,
	pub writer: Option<Box<dyn DataWriter>>,
}

impl SinkContext {
	pub fn new(data_kind: DataKind) -> Result<Self, DataWriterError> {
		let kind = data_kind.clone();
		let writer = match kind {
			DataKind::Local(url) => {
				todo!()
			},
			DataKind::Memory(url) => {
				todo!()
			},
			DataKind::Network(url) => {
				todo!()
			},
			_ => {
				println!("Unknown data kind: {:?}", kind);
				None
			}
		};
		Ok(Self {
			data_kind: data_kind,
			writer,
		})
	}

	pub fn url(&self) -> Option<&str> {
		match &self.data_kind {
			DataKind::Local(url) => Some(url.as_str()),
			DataKind::Memory(url) => Some(url.as_str()),
			DataKind::Network(url) => Some(url.as_str()),
			_ => None,
		}
	}

	pub fn writer_mut(&mut self) -> Option<&mut dyn DataWriter> {
		todo!()
	}
}
