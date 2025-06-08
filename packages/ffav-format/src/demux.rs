use crate::error::FormatError;
use crate::format_context::FormatContext;

pub trait Demux {
	fn read_probe(&self, ctx: &mut FormatContext) -> Result<(), FormatError>;
	fn read_header(&self, ctx: FormatContext) -> Result<(), FormatError>;
	fn read_packet(&self, ctx: FormatContext) -> Result<(), FormatError>;
	fn read_close(&self, ctx: FormatContext) -> Result<(), FormatError>;

	fn read_seek(&self, ctx: FormatContext, timestamp: i64, flags: i32) -> Result<(), FormatError>;
	fn read_timestamp(&self, ctx: FormatContext) -> Result<i64, FormatError>;

	fn read_play(&self, ctx: FormatContext) -> Result<(), FormatError>;
	fn read_pause(&self, ctx: FormatContext) -> Result<(), FormatError>;

	fn read_seek2(&self, ctx: FormatContext, timestamp: i64, flags: i32) -> Result<(), FormatError>;
}