use inventory;
use ffav_types::{ 
	CodecParameters,
	MediaPacket,
	MediaFrame,
	CodecID,
};
use crate::error::DecodeError;

pub trait DecoderInstance: Send + Sync {
	fn decode(&mut self, packet: &MediaPacket) -> Result<Option<MediaFrame>, DecodeError>;

	fn flush(&mut self) -> Result<Vec<MediaFrame>, DecodeError>;
}

pub trait Decoder: Send + Sync {
	fn name(&self) -> &'static str;

	fn codec_id(&self) -> CodecID;

	fn create(&self) -> Result<Box<dyn DecoderInstance>, DecodeError>;
} 

pub struct DecoderRegistry {
	pub decoder: &'static dyn Decoder,
}

inventory::collect!(DecoderRegistry);

#[macro_export]
macro_rules! register_decoder {
	(&decoder:expr) => {
		inventory::submit! {
			DecoderRegistry {
				decoder: &$decoder,
			}
		}
	};
}
