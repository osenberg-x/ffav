use inventory;

use ffav_types::{
	MediaPacket,
};
use crate::error::MuxError;

pub trait MuxerInstance: Send + Sync {
	fn write_packet(&mut self, packet: &MediaPacket) -> Result<(), MuxError>;
}

pub trait Muxer: Send + Sync {
	fn name(&self) -> &'static str;

	fn extensions(&self) -> &[&'static str];

	fn open(&self) -> Result<Box<dyn MuxerInstance>, MuxError>;
}

pub struct MuxerRegistry {
	pub muxer: &'static dyn Muxer,
}

inventory::collect!(MuxerRegistry);

#[macro_export]
macro_rules! register_muxer {
	(&muxer:expr) => {
		inventory::submit! {
			MuxerRegistry {
				muxer: &$muxer,
			}
		}
	};
}