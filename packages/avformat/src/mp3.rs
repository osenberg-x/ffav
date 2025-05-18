
#[derive(Debug)]
#[derive(Default)]
pub struct Mp3Header {
	version: u8,
	layer: u8,
	protection_bit: u8,
	bitrate_index: u8,
	sample_rate_index: u8,
	padding_bit: u8,
	private_bit: u8,
	mode_extension: u8,
	mode_extension_info: u8,
	copyright: u8,
	original: u8,
	emphasis: u8,	
}

impl Mp3Header {
	pub fn new(file_p : &str) -> Mp3Header {
		Mp3Header {
			..Default::default()
		}
	}
}