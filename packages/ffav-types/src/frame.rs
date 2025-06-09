use bytes::{Bytes, BytesMut};
use crate::frame_type::FrameType;
use crate::media_kind::MediaKind;

#[derive(Debug, Clone)]
pub struct Frame {
    data: Vec<Bytes>,
    linesizes: Vec<usize>,
    kind: MediaKind,
    pts: i64,
    width: usize,
    height: usize,
    format: usize,
    frame_type: FrameType,
}