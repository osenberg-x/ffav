use ffav_types::{ 
	CodecParameters,
	MediaPacket,
	MediaFrame,
};
use crate::error::CodecError;

pub trait Encoder {
    fn init(&mut self, codec_params: &CodecParameters) -> Result<(), CodecError>;
    fn encode(&mut self, frame: &MediaFrame) -> Result<Vec<MediaPacket>, CodecError>;
    fn flush(&mut self) -> Result<Vec<MediaPacket>, CodecError>;
}