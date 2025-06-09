use crate::pixel_format::PixelFormat;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct VideoMetadata {
    width: u32,
    height: u32,
    bit_rate: u64,
    pixel_format: PixelFormat,
}