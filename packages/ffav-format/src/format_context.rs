use crate::demuxer::Demuxer;


// #[derive(Debug)]
pub struct FormatContext {
    url: Option<String>,
    max_streams: u32,

	// AVIOContext *pb;  TODO

	demexer: Box<dyn Demuxer>,
}

impl FormatContext {
	pub fn open(&mut self, url: &str) -> Result<(), String> {
		// println!("url: {}", self.url.as_deref().unwrap_or(""));
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
