#[derive(Debug, Default)]
pub struct InputFormat {
	name: Option<String>,
	long_name: Option<String>,
	mime_type: Option<String>,
}

impl InputFormat {
	pub fn new() -> Self {
		Default::default()
	}

	pub fn from_name(name: &str) -> Self {
		let mut input_format = Self::default();
		input_format.name = Some(name.to_string());
		input_format
	}

	pub fn from_mime_type(mime_type: &str) -> Self {
		let mut input_format = Self::default();
		input_format.mime_type = Some(mime_type.to_string());
		input_format
	}

	pub fn name(&self) -> Option<&str> {
		self.name.as_deref()
	}

	pub fn long_name(&self) -> Option<&str> {
		self.long_name.as_deref()
	}

	pub fn mime_type(&self) -> Option<&str> {
		self.mime_type.as_deref()
	}
}
