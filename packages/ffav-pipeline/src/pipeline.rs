use std::{collections::HashMap, io::Sink};

use ffav_decode::{DecodeError, Decoder, DecoderInstance, DecoderRegistry, DecoderManager};
use ffav_demux::{DemuxError, Demuxer, DemuxerInstance, DemuxerRegistry, DemuxerManager};
use ffav_encode::{EncodeError, Encoder, EncoderInstance, EncoderRegistry, EncoderManager};
use ffav_filter::Filter;
use ffav_mux::{MuxError, Muxer, MuxerInstance, MuxerRegistry, MuxerManager};
use ffav_types::{CodecID, SourceContext, SinkContext};
use std::sync::OnceLock;

static DEMUXER_MANAGER: OnceLock<DemuxerManager> = OnceLock::new();
static MUXER_MANAGER: OnceLock<MuxerManager> = OnceLock::new();
static DECODER_MANAGER: OnceLock<DecoderManager> = OnceLock::new();
static ENCODER_MANAGER: OnceLock<EncoderManager> = OnceLock::new();

pub fn ffav_register_all() {
	DEMUXER_MANAGER.get_or_init(|| DemuxerManager::new());
	MUXER_MANAGER.get_or_init(|| MuxerManager::new());
	DECODER_MANAGER.get_or_init(|| DecoderManager::new());
	ENCODER_MANAGER.get_or_init(|| EncoderManager::new());
}

pub struct Pipeline {
	source: Option<SourceContext>,
	sink: Option<SinkContext>,
}

impl Pipeline {
    pub fn new() -> Self {
		Self {  
			source: None,
			sink: None,
		}
    }

	pub fn find_demuxer(&self) -> Option<&'static dyn Demuxer> {
		let data = b"1234";
		DEMUXER_MANAGER.get().unwrap().find_demuxer(data, None)
	}

	pub fn list_demuxers() {
		DEMUXER_MANAGER.get().unwrap().list_demuxers();
	}

	pub fn find_muxer(&self) -> Option<&'static dyn Muxer> {
		MUXER_MANAGER.get().unwrap().find_muxer("mp4")
	}

	pub fn list_muxers() {
		MUXER_MANAGER.get().unwrap().list_muxers();
	}

	pub fn find_decoder(&self) -> Option<&'static dyn Decoder> {
		DECODER_MANAGER.get().unwrap().find_decoder(CodecID::WAV)
	}

	pub fn list_decoders() {
		DECODER_MANAGER.get().unwrap().list_decoders();
	}

	pub fn find_encoder(&self) -> Option<&'static dyn Encoder> {
		ENCODER_MANAGER.get().unwrap().find_encoder("H265")
	}

	pub fn list_encoders() {
		ENCODER_MANAGER.get().unwrap().list_encoders();
	}

}
