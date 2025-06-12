use ffav_codec::codec::Codec;

#[derive(Debug)]
pub struct FormatContext {
    url: Option<String>,
    max_streams: u32,

	// AVIOContext *pb;  TODO

    // 0 - video codecs, 1 - audio codecs, 2 - subtitle codecs, 3 - data codecs
    codecs: Vec<Codec>,
}

impl FormatContext {
	pub fn new(url: &str) -> Self {
		Self {
			url: Some(url.to_string()),
			max_streams: 0,
			codecs: Vec::new(),
		}
	}

	pub fn open(&mut self) -> Result<(), String> {
		Ok(())
	}

	fn find_stream_info(&mut self) -> Result<(), String> {
		Ok(())
	}

	fn close(&mut self) -> Result<(), String> {
		self.url = None;
		Ok(())
	}
}

impl Drop for FormatContext {
	fn drop(&mut self) {
		if let Some(url) = &self.url {
			println!("Closing format context for {}", url);
		}
		self.close().unwrap();
	}
}
