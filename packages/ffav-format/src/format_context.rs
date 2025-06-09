use ffav_codec::codec::Codec;

#[derive(Debug)]
pub struct FormatContext {
    url: Option<String>,
    max_streams: u32,

    // 0 - video codecs, 1 - audio codecs, 2 - subtitle codecs, 3 - data codecs
    codecs: Vec<Codec>,
}

impl FormatContext {
	fn open(&mut self, url: &str) -> Result<(), String> {
		self.url = Some(url.to_string());
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
