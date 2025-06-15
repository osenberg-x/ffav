use ffav_types::MediaPacket;

use crate::error::FormatError;
use crate::format_context::FormatContext;

pub trait Demuxer: Send + Sync {
	// default open
	// fn open(&mut self, ctx: &mut FormatContext) -> Result<(), FormatError>;
	// fn close(&mut self, ctx: &mut FormatContext) -> Result<(), FormatError>;

	fn format_probe(&mut self, ctx: &mut FormatContext) -> Result<(), FormatError>;

	fn format_header(&mut self, ctx: &mut FormatContext) -> Result<(), FormatError>;

	fn read_packet(&mut self, packet: &mut MediaPacket) -> Result<(), FormatError>;
}