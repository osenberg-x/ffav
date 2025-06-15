use crate::pixel_format::PixelFormat;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct VideoAttribute {
    width: Option<u32>,
    height: Option<u32>,
    bit_rate: Option<u64>,
    pixel_format: Option<PixelFormat>,
}

impl VideoAttribute {
    pub fn new(width: Option<u32>, height: Option<u32>, bit_rate: Option<u64>, pixel_format: Option<PixelFormat>) -> Self {
        Self {
            width,
            height,
            bit_rate,
            pixel_format,
        }
    }
}