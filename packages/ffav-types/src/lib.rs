// Defines media type enumerations and related utilities
pub mod media_kind;
pub use media_kind::*;

// Represents media packet data and metadata
pub mod media_packet;
pub use media_packet::*;
// Defines enumerations for different media frame types
pub mod frame_type;
pub use frame_type::*;
// Contains media frame data structures and operations
pub mod media_frame;
pub use media_frame::*;

// Enumerates supported pixel formats for video data
pub mod pixel_format;
pub use pixel_format::*;

// Defines audio sample formats and properties
pub mod sample_format;
pub use sample_format::*;
// Defines audio stream properties and attributes
pub mod video_attribute;
pub use video_attribute::*;
// Defines audio stream properties and attributes
pub mod audio_attribute;
pub use audio_attribute::*;
// Stores codec-specific parameters and configurations
pub mod codec_parameters;
pub use codec_parameters::*;
// Enumerates supported codec identifiers
pub mod codec_id;
pub use codec_id::*;

// Implements rational number arithmetic for media calculations
pub mod rational;
pub use rational::*;

pub mod stream_attribute;
pub use stream_attribute::{StreamAttribute};

pub mod format_flag;
pub use format_flag::{
	FormatFlag,
};

pub mod data_kind;
pub use data_kind::DataKind;

pub mod data_chunk;
pub use data_chunk::DataChunk;

pub mod input_metadata;
pub use input_metadata::InputMetadata;

pub mod data_reader;
pub use data_reader::*;

pub mod data_writer;
pub use data_writer::*;

pub mod source_context;
pub use source_context::SourceContext;

pub mod sink_context;
pub use sink_context::SinkContext;

#[cfg(test)]
mod tests {
	use super::*;
	use std::path::Path;

	#[test]
	fn test_data_reader() {
		let mut file_reader = FileReader::new(Path::new("E:/ws/ffav/test.txt")).unwrap();
		let buffer = file_reader.read_chunk(4).unwrap();
		println!("read buffer: {:?}", buffer);
	}
}