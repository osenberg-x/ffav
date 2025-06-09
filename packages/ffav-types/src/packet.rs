use bytes::{Bytes, BytesMut};

#[derive(Debug, Clone)]
pub struct Packet {
	data: Vec<Bytes>,
	stream_index: i32,
	pts: i64,
	dts: i64,
	pos: i64,
	flags: i32,
	duration: i64,
	size: usize,
	side_data: Vec<Bytes>,
	side_data_size: usize,
}