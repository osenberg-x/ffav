use ffav_types::{
	MediaPacket,
	StreamAttribute,
};

use crate::error::DemuxError;

pub trait Demuxer: Send + Sync {
	// read probe information
	fn read_probe(&mut self) -> Result<(), DemuxError>;

	// read header of stream
	fn read_header(&mut self) -> Result<(), DemuxError>;

	// read data packet
	fn read_packet(&mut self, packet: &mut MediaPacket) -> Result<(), DemuxError>;

	fn get_stream_attribute(&mut self, index: usize) -> Option<&StreamAttribute>;
}