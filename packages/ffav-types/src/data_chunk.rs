use arc_slice::layout::{ArcLayout, BoxedSliceLayout, DefaultLayout, VecLayout};
use arc_slice::{ArcBytes, ArcBytesMut};

#[derive(Debug)]
pub struct DataChunk {
    pub data: ArcBytes::<BoxedSliceLayout>,
}
