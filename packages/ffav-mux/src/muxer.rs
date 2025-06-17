use crate::error::MuxError;
use ffav_types::media_packet::MediaPacket;

pub trait Muxer {
	// fn create(&mut self, output: &mut dyn Write, format_context: &FormatContext) -> Result<()>;
	fn open(&mut self, output_url: &str) -> Result<(), MuxError>;
    fn write_packet(&mut self, packet: &MediaPacket) -> Result<(), MuxError>;
    fn close(&mut self) -> Result<(), MuxError>;
}