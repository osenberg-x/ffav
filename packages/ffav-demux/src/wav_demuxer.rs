use std::{fs::File, io::{BufReader, Read}};
use crate::demuxer::Demuxer;
use crate::error::DemuxError;
use ffav_types::{
	MediaPacket,
	StreamAttribute,
};

#[derive(Debug)]
#[derive(Default)]
pub struct WavHeader {
	chunk_id: String,
	chunk_size: u32,
	format: String,
	subchunk1_id: String,
	subchunk1_size: u32,
	audio_format: u16,
	num_channels: u16,
	sample_rate: u32,
	byte_rate: u32,
	block_align: u16,
	bits_per_sample: u16,
	subchunk2_id: String,
	subchunk2_size: u32,
}

impl WavHeader {
	pub fn new(file : &mut File) -> WavHeader {
		let mut buffer = [0u8; 44];
		file.read_exact(&mut buffer).unwrap();
	
		let mut offset = 0;
	
		let chunk_id = String::from_utf8(buffer[offset..offset + 4].to_vec()).unwrap();
		offset += 4;
	
		let chunk_size = u32::from_be_bytes(buffer[offset..offset + 4].try_into().unwrap());
		offset += 4;
	
		let format = String::from_utf8(buffer[offset..offset + 4].to_vec()).unwrap();
		offset += 4;
	
		let subchunk1_id = String::from_utf8(buffer[offset..offset + 4].to_vec()).unwrap();
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
	
		let subchunk2_id = String::from_utf8(buffer[offset..offset + 4].to_vec()).unwrap();
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

pub struct WavDemuxer {
	handle: File,
	header: WavHeader,
}

impl Demuxer for WavDemuxer {
	fn open(&mut self, url: &str) -> Result<(), DemuxError> {
		let rs = File::open(url);
		match rs {
			Ok(f) => {
				self.handle = f;
			}
			Err(e) => {
				return Err(DemuxError::OpenFileError);
			}
		}
		let header = WavHeader::new(&mut self.handle);
		self.header = header;
		Ok(())
	}
	
	fn read_probe(&mut self) -> Result<(), DemuxError> {
		Ok(())
	}

	fn read_header(&mut self) -> Result<(), DemuxError> {
		Ok(())
	}

	fn read_packet(&mut self, packet: &mut MediaPacket) -> Result<(), DemuxError> {
		Ok(())
	}

	fn get_stream_attribute(&mut self, index: usize) -> Option<&StreamAttribute> {
		None
	}
}
