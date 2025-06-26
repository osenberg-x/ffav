use arc_slice::layout::{BoxedSliceLayout};
use arc_slice::{ArcBytes};

#[derive(Debug)]
pub struct DataChunk {
    pub data: ArcBytes::<BoxedSliceLayout>,
}
