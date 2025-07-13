use inventory;

use ffav_types::{
	MediaPacket,
	StreamAttribute,
};

use crate::error::DemuxError;

pub trait DemuxerInstance: Send + Sync {
	fn read_packet(&mut self) -> Result<MediaPacket, DemuxError>;

	fn seek(&mut self, timestamp: u64) -> Result<(), String>;
}

pub trait Demuxer: Send + Sync {
	fn name(&self) -> &'static str;

	fn extensions(&self) -> &[&'static str];

	fn header_size(&self) -> usize;

	fn probe(&self, data: &[u8]) -> u32;

	fn open(&self) -> Result<Box<dyn DemuxerInstance>, DemuxError>;
}

pub struct DemuxerRegistry {
	pub demuxer: &'static dyn Demuxer,
}

inventory::collect!(DemuxerRegistry);

#[macro_export]
macro_rules! register_demuxer {
	($demuxer:expr) => {
		inventory::submit! {
			DemuxerRegistry {
				demuxer: &$demuxer,
			}
		}
	};
}
