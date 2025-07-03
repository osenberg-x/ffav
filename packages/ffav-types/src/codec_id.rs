#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CodecID {
    NONE,

    /* video codecs */
    H264,
    H265,
    VP8,
    VP9,
    AAC,

    MP4,

    MP3,
    OPUS,
    WAV,
}