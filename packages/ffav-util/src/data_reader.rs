use arc_slice::{ArcBytesMut};

#[derive(Debug)]
pub struct DataReader {
	buffer: ArcBytesMut,
}
