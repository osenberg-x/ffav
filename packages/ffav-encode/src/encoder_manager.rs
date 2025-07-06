use std::collections::HashMap;
use inventory;
use crate::{
	Encoder,
	EncoderRegistry
};

pub struct EncoderManager {
	encoders: HashMap<&'static str, &'static dyn Encoder>,
}

impl EncoderManager {
	pub fn new() -> Self {
		let mut manager = Self {
			encoders: HashMap::new(),
		};

		manager.init();
		manager
	}

	fn init(&mut self) {
        // collect all registered encoders
        for entry in inventory::iter::<EncoderRegistry> {
            self.encoders.insert(entry.encoder.name(), entry.encoder);
        }

        println!("registered {} encoders.", self.encoders.len());
	}

	pub fn find_encoder(&self, name: &str) -> Option<&'static dyn Encoder> {
		self.encoders.get(name).copied()
	}

	pub fn list_encoders(&self) {
		println!("avaliable {} encoders", self.encoders.len());
		for (name, encoder) in &self.encoders {
			println!("    - {}: {:?}", name, encoder.formats());
		}
	}
}
