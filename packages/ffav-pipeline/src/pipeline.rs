use inventory;
use std::{collections::HashMap};

use ffav_demux::{
	Demuxer,
	DemuxerRegistry,
	DemuxerInstance,
	DemuxError
};
use ffav_mux::{
	MuxerRegistry, 
	Muxer, 
	MuxerInstance,
	MuxError
};
use ffav_decode::{
	Decoder,
	DecoderInstance,
	DecoderRegistry,
	DecodeError
};
use ffav_encode::{
	Encoder,
	EncoderInstance,
	EncoderRegistry,
	EncodeError
};
use ffav_filter::Filter;
use ffav_types::CodecID;

pub struct Pipeline {
	demuxers: Vec<&'static dyn Demuxer>,
	// todo str use enum?
	muxers: HashMap<&'static str, &'static dyn Muxer>,
	decoders: HashMap<CodecID, &'static dyn Decoder>,
	encoders: HashMap<&'static str, &'static dyn Encoder>,
	filters: HashMap<&'static str, &'static dyn Filter>,
}

impl Pipeline {
	pub fn new() -> Self {
		let mut pipeline = Self {
			demuxers: Vec::new(),
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
			self.demuxers.push(entry.demuxer);
		}

		// collect all registered muxers
		for entry in inventory::iter::<MuxerRegistry> {
			self.muxers.insert(entry.muxer.name(), entry.muxer);
		}

		// collect all registered decoders
		for entry in inventory::iter::<DecoderRegistry> {
			self.decoders.insert(entry.decoder.codec_id(), entry.decoder);
		}

		// collect all registered encoders
		for entry in inventory::iter::<EncoderRegistry> {
			self.encoders.insert(entry.encoder.name(), entry.encoder);
		}
		
		// collect all registered filters
		// todo
	}
}