pub mod args;
pub use args::Args;

pub mod data_reader;
pub use data_reader::*;

pub mod data_writer;
pub use data_writer::DataWriter;

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