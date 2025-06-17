pub mod error;
pub use error::DemuxError;

pub mod demuxer;
pub use demuxer::Demuxer;

pub mod wav_demuxer;
pub use wav_demuxer::WavHeader;