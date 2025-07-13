use std::mem;
use crate::{
	register_demuxer,
	DemuxError, 
	Demuxer, 
	DemuxerInstance, 
	DemuxerRegistry
};
use ffav_types::{
	MediaPacket,
	DataChunk,
};

#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct WavHeader {
	chunk_id: [u8; 4],
	chunk_size: u32,
	format: [u8; 4],
	subchunk1_id: [u8; 4],
	subchunk1_size: u32,
	audio_format: u16,
	num_channels: u16,
	sample_rate: u32,
	byte_rate: u32,
	block_align: u16,
	bits_per_sample: u16,
	subchunk2_id: [u8; 4],
	subchunk2_size: u32,
}

impl WavHeader {
	pub fn new() -> WavHeader {
		WavHeader::default()
	}

	pub fn from_data(chunk : &[u8]) -> WavHeader {
		let buffer = chunk;

		let mut offset = 0;
	
		let chunk_id = buffer[offset..offset + 4].try_into().unwrap();
		offset += 4;
	
		let chunk_size = u32::from_be_bytes(buffer[offset..offset + 4].try_into().unwrap());
		offset += 4;
	
		let format = buffer[offset..offset + 4].try_into().unwrap();
		offset += 4;
	
		let subchunk1_id = buffer[offset..offset + 4].try_into().unwrap();
		offset += 4;
	
		let subchunk1_size = u32::from_le_bytes(buffer[offset..offset + 4].try_into().unwrap());
		offset += 4;
	
		let audio_format = u16::from_le_bytes(buffer[offset..offset + 2].try_into().unwrap());
		offset += 2;
	
		let num_channels = u16::from_le_bytes(buffer[offset..offset + 2].try_into().unwrap());
		offset += 2;
	
		let sample_rate = u32::from_le_bytes(buffer[offset..offset + 4].try_into().unwrap());
		offset += 4;
	
		let byte_rate = u32::from_le_bytes(buffer[offset..offset + 4].try_into().unwrap());
		offset += 4;
	
		let block_align = u16::from_le_bytes(buffer[offset..offset + 2].try_into().unwrap());
		offset += 2;
	
		let bits_per_sample = u16::from_le_bytes(buffer[offset..offset + 2].try_into().unwrap());
		offset += 2;
	
		let subchunk2_id = buffer[offset..offset + 4].try_into().unwrap();
		offset += 4;
	
		let subchunk2_size = u32::from_le_bytes(buffer[offset..offset + 4].try_into().unwrap());
	
		WavHeader {
		    chunk_id,
		    chunk_size,
		    format,
		    subchunk1_id,
		    subchunk1_size,
		    audio_format,
		    num_channels,
		    sample_rate,
		    byte_rate,
		    block_align,
		    bits_per_sample,
		    subchunk2_id,
		    subchunk2_size,
		}
	}
}

pub struct WavDemuxerInstance {
	pub header: WavHeader,
}

impl WavDemuxerInstance {
	fn new() -> WavDemuxerInstance {
		WavDemuxerInstance {
			header: WavHeader::new(),
		}
	}
}

impl DemuxerInstance for WavDemuxerInstance {
	fn read_packet(&mut self) -> Result<MediaPacket, DemuxError> {
		todo!()
	}

	fn seek(&mut self, _timestamp: u64) -> Result<(), String> {
		todo!()
	}
}

pub struct WavDemuxer;

impl Demuxer for WavDemuxer {
	fn name(&self) -> &'static str {
		"W64-Demuxer"
	}

	fn extensions(&self) -> &[&'static str] {
		&["wav"]
	}

	fn header_size(&self) -> usize {
		mem::size_of::<WavHeader>()
	}

	fn probe(&self, data: &[u8]) -> u32 {
		let header = WavHeader::from_data(data);
		if header.chunk_id != [b'R', b'I', b'F', b'F'] {
			return 0;
		}
		if header.format != [b'W', b'A', b'V', b'E'] {
			return 0;
		}
		10
	}

	fn open(&self) -> Result<Box<dyn DemuxerInstance>, DemuxError> {
		Ok(Box::new(WavDemuxerInstance::new()))
	}
}

static WAV_DEMUXER: WavDemuxer = WavDemuxer;

register_demuxer!(WAV_DEMUXER);
