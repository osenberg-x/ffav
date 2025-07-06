pub mod error;
pub use error::DemuxError;

pub mod demuxer;
pub use demuxer::{
	Demuxer,
	DemuxerInstance,
	DemuxerRegistry,
};

pub mod demuxer_manager;
pub use demuxer_manager::DemuxerManager;

pub mod wav_demuxer;
pub use wav_demuxer::WavHeader;