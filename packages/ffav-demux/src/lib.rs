pub mod error;
pub use error::DemuxError;

pub mod demuxer;
pub use demuxer::{
	Demuxer,
	DemuxerInstance,
	DemuxerRegistry
};

pub mod demuxer_context;
pub use demuxer_context::DemuxerContext;

pub mod wav_demuxer;
pub use wav_demuxer::WavHeader;