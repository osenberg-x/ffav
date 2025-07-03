use std::{collections::HashMap, hash::Hash};

use ffav_demux::Demuxer;
use ffav_mux::Muxer;
use ffav_filter::Filter;
use ffav_decode::Decoder;
use ffav_encode::Encoder;

pub struct Pipeline {
	demuxers: Vec<&'static dyn Demuxer>,
	muxers: HashMap<&'static str, Box<dyn Muxer>>,
	filters: HashMap<&'static str, Box<dyn Filter>>,
	decoders: HashMap<&'static str, Box<dyn Decoder>>,
	encoders: HashMap<&'static str, Box<dyn Encoder>>,
}

impl Pipeline {
	pub fn new() -> Self {
		Self {
			demuxers: Vec::new(),
			muxers: HashMap::new(),
			filters: HashMap::new(),
			decoders: HashMap::new(),
			encoders: HashMap::new(),
		}
	}
}