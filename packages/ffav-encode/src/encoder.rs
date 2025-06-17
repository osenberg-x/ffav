use ffav_types::{ 
	CodecParameters,
	MediaPacket,
	MediaFrame,
};
use crate::error::EncodeError;

pub trait Encoder {
    fn init(&mut self, codec_params: &CodecParameters) -> Result<(), EncodeError>;
    fn encode(&mut self, frame: &MediaFrame) -> Result<Vec<MediaPacket>, EncodeError>;
    fn flush(&mut self) -> Result<Vec<MediaPacket>, EncodeError>;
}