use inventory;
use std::{collections::HashMap, hash::Hash};

use ffav_decode::{DecodeError, Decoder, DecoderInstance, DecoderRegistry};
use ffav_demux::{DemuxError, Demuxer, DemuxerInstance, DemuxerRegistry};
use ffav_encode::{EncodeError, Encoder, EncoderInstance, EncoderRegistry};
use ffav_filter::Filter;
use ffav_mux::{MuxError, Muxer, MuxerInstance, MuxerRegistry};
use ffav_types::CodecID;

pub struct Pipeline {
    demuxers: HashMap<&'static str, &'static dyn Demuxer>,
    // todo str use enum?
    muxers: HashMap<&'static str, &'static dyn Muxer>,
    decoders: HashMap<CodecID, &'static dyn Decoder>,
    encoders: HashMap<&'static str, &'static dyn Encoder>,
    filters: HashMap<&'static str, &'static dyn Filter>,
}

impl Pipeline {
    pub fn new() -> Self {
        let mut pipeline = Self {
            demuxers: HashMap::new(),
            muxers: HashMap::new(),
            filters: HashMap::new(),
            decoders: HashMap::new(),
            encoders: HashMap::new(),
        };
        pipeline.init();
        pipeline
    }

    fn init(&mut self) {
        // collect all registered demuxers
        for entry in inventory::iter::<DemuxerRegistry> {
            self.demuxers.insert(entry.demuxer.name(), entry.demuxer);
        }

        // collect all registered muxers
        for entry in inventory::iter::<MuxerRegistry> {
            self.muxers.insert(entry.muxer.name(), entry.muxer);
        }

        // collect all registered decoders
        for entry in inventory::iter::<DecoderRegistry> {
            self.decoders
                .insert(entry.decoder.codec_id(), entry.decoder);
        }

        // collect all registered encoders
        for entry in inventory::iter::<EncoderRegistry> {
            self.encoders.insert(entry.encoder.name(), entry.encoder);
        }

        // collect all registered filters
        // todo

        println!("Registered {} demuxers.", self.demuxers.len());
        println!("Registered {} muxers.", self.muxers.len());
        println!("Registered {} decoders.", self.decoders.len());
        println!("Registered {} encoders.", self.encoders.len());
        println!("Registered {} filters.", self.filters.len());
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

    pub fn find_muxer(&self, name: &str) -> Option<&'static dyn Muxer> {
		self.muxers.get(name).copied()
    }

	pub fn list_muxers(&self) {
		println!("avaliable {} muxers", self.muxers.len());

		for (name, muxer) in &self.muxers {
			println!("    - {}: {:?}", name, muxer.extensions());
		}
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
