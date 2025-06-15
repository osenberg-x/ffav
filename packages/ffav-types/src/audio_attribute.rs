use crate::sample_format::{self, SampleFormat};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AudioAttribute {
    sample_rate: Option<u32>,
    channels: Option<u32>,
    sample_format: Option<SampleFormat>,
    nb_samples: Option<usize>
}

impl AudioAttribute {
    pub fn new(sample_rate: Option<u32>, channels: Option<u32>, sample_format: Option<SampleFormat>, nb_samples: Option<usize>) -> Self {
        Self {
            sample_rate,
            channels,
            sample_format,
            nb_samples,
        }
    }
}