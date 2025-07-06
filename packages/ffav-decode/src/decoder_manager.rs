use std::collections::HashMap;
use inventory;
use crate::{
	Decoder,
	DecoderRegistry
};
use ffav_types::CodecID;

pub struct DecoderManager {
	pub decoders: HashMap<CodecID, &'static dyn Decoder>,
}

impl DecoderManager {
	pub fn new() -> Self {
		let mut manager =  Self {
			decoders: HashMap::new(),
		};

		manager.init();
		manager
	}

	fn init(&mut self) {
		for entry in inventory::iter::<DecoderRegistry> {
			self.decoders.insert(entry.decoder.codec_id(), entry.decoder);
		}

        println!("Registered {} decoders.", self.decoders.len());
	}

	pub fn find_decoder(&self, codec_id: CodecID) -> Option<&'static dyn Decoder> {
        self.decoders.get(&codec_id).copied()
    }

	pub fn list_decoders(&self) {
		println!("avaliable {} decoders", self.decoders.len());

		for (codec_id, decoder) in &self.decoders {
			println!("    - {:?}: {:?}", codec_id, decoder.name());
		}
	}
}