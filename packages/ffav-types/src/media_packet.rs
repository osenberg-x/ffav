use arc_slice::ArcBytesMut;
use crate::frame_type::FrameType;

#[derive(Debug)]
pub struct MediaPacket {
	data: ArcBytesMut,
	stream_index: i32,
	pts: i64,
	dts: Option<i64>,
	// is_key_frame: bool,
	frame_type: FrameType,
	duration: Option<i64>,
	side_data: ArcBytesMut,
}