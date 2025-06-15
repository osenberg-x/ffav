use ffav_types::{ 
	CodecParameters,
	MediaPacket,
	MediaFrame,
};
use crate::error::CodecError;

pub trait Decoder {
	fn init(&mut self, codec_params: &CodecParameters) -> Result<(), CodecError>;
    fn decode(&mut self, packet: &MediaPacket) -> Result<Vec<MediaFrame>, CodecError>;
    fn flush(&mut self) -> Result<Vec<MediaFrame>, CodecError>;
} 
// Codec {
// 	name: String,
// 	long_name: String,
// 	id: CodecID,
// 	kind: MediaKind,
// }