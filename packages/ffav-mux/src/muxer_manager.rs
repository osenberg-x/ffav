use std::collections::HashMap;
use inventory;
use crate::{
	Muxer,
	MuxerRegistry
};

pub struct MuxerManager {
	muxers: HashMap<&'static str, &'static dyn Muxer>
}

impl MuxerManager {
	pub fn new() -> Self {
		let mut manager = Self {
			muxers: HashMap::new(),
		};

		manager.init();
		manager
	}

	fn init(&mut self) {
        // collect all registered muxers
        for entry in inventory::iter::<MuxerRegistry> {
            self.muxers.insert(entry.muxer.name(), entry.muxer);
        }

        println!("registered {} muxers.", self.muxers.len());
	}

    pub fn find_muxer(&self, name: &str) -> Option<&'static dyn Muxer> {
		self.muxers.get(name).copied()
    }

	pub fn list_muxers(&self) {
		println!("avaliable {} muxers", self.muxers.len());

		for (name, muxer) in &self.muxers {
			println!("    - {}: {:?}", name, muxer.extensions());
		}
	}
}
