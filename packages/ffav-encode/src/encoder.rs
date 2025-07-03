use inventory;
use ffav_types::{ 
	CodecParameters,
	MediaPacket,
	MediaFrame,
	CodecID,
};
use crate::error::EncodeError;

pub trait EncoderInstance: Send + Sync {
	fn encode(&mut self, frame: &MediaFrame) -> Result<Option<MediaPacket>, EncodeError>;

    fn flush(&mut self) -> Result<Vec<MediaPacket>, EncodeError>;
}

pub trait Encoder: Send + Sync {
	fn name(&self) -> &'static str;

	// fn codec_id(&self) -> CodecID;
	fn supported_formats(&self) -> &[&'static str];

	fn create(&self) -> Result<Box<dyn EncoderInstance>, EncodeError>;
}

pub struct EncoderRegistry {
	pub encoder: &'static dyn Encoder,
}

inventory::collect!(EncoderRegistry);

#[macro_export]
macro_rules! register_encoder {
	(&encoder:expr) => {
		inventory::submit! {
			EncoderRegistry {
				encoder: &$encoder,
			}
		}
	};
}