use std::collections::HashMap;

use inventory;
use crate::{
	Demuxer,
	DemuxerRegistry,
    DemuxError
};
use ffav_types::{ 
    DataReader,
    DataReaderError,
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

        println!("registered {} demuxers.", self.demuxers.len());
	}

    pub fn find_demuxer(&self, data_reader: &mut Box<dyn DataReader>, url: Option<&str>) -> Result<&'static dyn Demuxer, DemuxError> {
        let mut best_demuxer = None;
        let mut best_score = 10;

        self.demuxers.iter().for_each(|(_name, demuxer)| {
            let probe_size: u64 = demuxer.header_size() as u64;
            // let data = data_reader.read_chunk(probe_size)?;
            let data = data_reader.read_chunk(probe_size).expect("failed to read chunk");

            let mut score = demuxer.probe(data.data.as_slice());

            if let Some(url) = url {
                if let Some(extension) = url.split(".").last() {
                    if demuxer.extensions().contains(&extension) {
                        score += 10;
                    }
                }
            }

            if score >= best_score {
                best_score = score;
                best_demuxer = Some(demuxer);
            } else {
                data_reader.seek(probe_size as i64).expect("failed to seek reader");
            }
        });

        Ok(best_demuxer.copied().ok_or(DemuxError::NoDemuxerFound)?)
    }

	pub fn list_demuxers(&self) {
		println!("avaliable {} demuxers", self.demuxers.len());

		for (name, demuxer) in &self.demuxers {
			println!("    - {}: {:?}", name, demuxer.extensions());
		}
	}

}
