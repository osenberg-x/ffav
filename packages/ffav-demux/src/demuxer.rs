use ffav_types::{
	MediaPacket,
	StreamAttribute,
};

use crate::error::DemuxError;

pub trait Demuxer: Send + Sync {
	// default open
	// TODO(open file or open memory or open url)
	fn open(&mut self, url: &str) -> Result<(), DemuxError>;
	// fn close(&mut self, ctx: &mut FormatContext) -> Result<(), FormatError>;

	// read probe information
	fn read_probe(&mut self) -> Result<(), DemuxError>;

	// read header of stream
	fn read_header(&mut self) -> Result<(), DemuxError>;

	// read data packet
	fn read_packet(&mut self, packet: &mut MediaPacket) -> Result<(), DemuxError>;

	fn get_stream_attribute(&mut self, index: usize) -> Option<&StreamAttribute>;
}