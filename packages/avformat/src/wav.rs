use std::{fs::File, io::{BufReader, Read}};

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
	// pub fn new(file_p : &str) -> WavHeader {
	// 	let mut file = File::open(file_p).unwrap();
	// 	let mut chunk_id = [0u8; 4];
	// 	file.read_exact(&mut chunk_id).unwrap();

	// 	let mut chunk_size = [0u8; 4];
	// 	file.read_exact(&mut chunk_size).unwrap();

	// 	let mut format = [0u8; 4];
	// 	file.read_exact(&mut format).unwrap();

	// 	let mut subchunk1_id = [0u8; 4];
	// 	file.read_exact(&mut subchunk1_id).unwrap();

	// 	let mut subchunk1_size = [0u8; 4];
	// 	file.read_exact(&mut subchunk1_size).unwrap();

	// 	let mut audio_format = [0u8; 2];
	// 	file.read_exact(&mut audio_format).unwrap();

	// 	let mut num_channels = [0u8; 2];
	// 	file.read_exact(&mut num_channels).unwrap();

	// 	let mut sample_rate = [0u8; 4];
	// 	file.read_exact(&mut sample_rate).unwrap();

	// 	let mut byte_rate = [0u8; 4];
	// 	file.read_exact(&mut byte_rate).unwrap();

	// 	let mut block_align = [0u8; 2];
	// 	file.read_exact(&mut block_align).unwrap();

	// 	let mut bits_per_sample = [0u8; 2];
	// 	file.read_exact(&mut bits_per_sample).unwrap();

	// 	let mut subchunk2_id = [0u8; 4];
	// 	file.read_exact(&mut subchunk2_id).unwrap();

	// 	let mut subchunk2_size = [0u8; 4];
	// 	file.read_exact(&mut subchunk2_size).unwrap();

	// 	WavHeader {
	// 		chunk_id: String::from_utf8(chunk_id.to_vec()).unwrap(),
	// 		chunk_size: u32::from_be_bytes(chunk_size),
	// 		format: String::from_utf8(format.to_vec()).unwrap(),
	// 		subchunk1_id: String::from_utf8(subchunk1_id.to_vec()).unwrap(),
	// 		subchunk1_size: u32::from_be_bytes(subchunk1_size),
	// 		audio_format: u16::from_be_bytes(audio_format),
	// 		num_channels: u16::from_be_bytes(num_channels),
	// 		sample_rate: u32::from_be_bytes(sample_rate),
	// 		byte_rate: u32::from_be_bytes(byte_rate),
	// 		block_align: u16::from_be_bytes(block_align),
	// 		bits_per_sample: u16::from_be_bytes(bits_per_sample),
	// 		subchunk2_id: String::from_utf8(subchunk2_id.to_vec()).unwrap(),
	// 		subchunk2_size: u32::from_be_bytes(subchunk2_size),
	// 	}
	// }
	pub fn new(file_p : &str) -> WavHeader {
		let mut file = File::open(file_p).unwrap();
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