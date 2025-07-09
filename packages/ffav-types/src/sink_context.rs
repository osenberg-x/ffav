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
}
