// This code was inspired by and/or derived from [Project Name]
// ([https://github.com/rust-av/rust-av]). See the original source for more details.

use arc_slice::{ArcSlice, ArcBytes, ArcBytesMut};
use crate::frame_type::FrameType;
use crate::media_kind::MediaKind;
use std::sync::Arc;

#[derive(Debug)]
struct DataPlane {
    data: ArcBytesMut,
    linesize: i32,
}

#[derive(Debug)]
pub struct MediaFrame {
    data: Arc<Vec<DataPlane>>,
    timestamp: Option<i64>,
    duration: Option<i64>,
    kind: MediaKind,
}

impl MediaFrame {
    pub fn new(kind: &MediaKind, timestamp: Option<i64>, duration: Option<i64>) -> Self {
        Self {
            data: Arc::new(Vec::new()),
            timestamp,
            duration,
            kind: *kind,
        }
    }
}