use crate::error::FormatError;
use crate::format_context::FormatContext;
use ffav_types::media_packet::MediaPacket;

pub trait Muxer {
	// fn create(&mut self, output: &mut dyn Write, format_context: &FormatContext) -> Result<()>;
	fn create(&mut self, format_context: &FormatContext) -> Result<(), FormatError>;
    fn write_packet(&mut self, packet: &MediaPacket) -> Result<(), FormatError>;
    fn finalize(&mut self) -> Result<(), FormatError>;
}