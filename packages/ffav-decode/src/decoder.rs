use ffav_types::{ 
	CodecParameters,
	MediaPacket,
	MediaFrame,
};
use crate::error::DecodeError;

pub trait Decoder {
	fn init(&mut self, codec_params: &CodecParameters) -> Result<(), DecodeError>;
    fn decode(&mut self, packet: &MediaPacket) -> Result<Vec<MediaFrame>,DecodeError>;
    fn flush(&mut self) -> Result<Vec<MediaFrame>, DecodeError>;
} 
// Codec {
// 	name: String,
// 	long_name: String,
// 	id: CodecID,
// 	kind: MediaKind,
// }