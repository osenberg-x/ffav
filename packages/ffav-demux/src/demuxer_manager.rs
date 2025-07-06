use std::collections::HashMap;

use inventory;
use crate::{
	Demuxer,
	DemuxerRegistry
};

pub struct DemuxerManager {
	demuxers: HashMap<&'static str, &'static dyn Demuxer>
}

impl DemuxerManager {
    pub fn new() -> Self {
        let mut manager = Self {
            demuxers: HashMap::new(),
        };

		manager.init();
		manager
    }

    fn init(&mut self) {
        // collect all registered demuxers
        for entry in inventory::iter::<DemuxerRegistry> {
            self.demuxers.insert(entry.demuxer.name(), entry.demuxer);
        }

        println!("Registered {} demuxers.", self.demuxers.len());
	}

    pub fn find_demuxer(&self, data: &[u8], url: Option<&str>) -> Option<&'static dyn Demuxer> {
        let mut best_demuxer = None;
        let mut best_score = 10;

        self.demuxers.iter().for_each(|(_name, demuxer)| {
            let mut score = demuxer.probe(data);

            if let Some(url) = url {
                if let Some(extension) = url.split(".").last() {
                    if demuxer.extensions().contains(&extension) {
                        score += 10;
                    }
                }
            }

            if score < best_score {
                best_score = score;
                best_demuxer = Some(demuxer);
            }
        });

        best_demuxer.copied()
    }

	pub fn list_demuxers(&self) {
		println!("avaliable {} demuxers", self.demuxers.len());

		for (name, demuxer) in &self.demuxers {
			println!("    - {}: {:?}", name, demuxer.extensions());
		}
	}

}
