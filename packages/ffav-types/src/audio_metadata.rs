use crate::sample_format::SampleFormat;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AudioMetadata {
    sample_rate: i32,
    channels: i32,
    sample_format: SampleFormat,
}