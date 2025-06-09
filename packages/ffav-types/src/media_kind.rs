use crate::{audio_metadata::AudioMetadata, video_metadata::VideoMetadata};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum MediaKind {
    /// Usually treated as AVMEDIA_TYPE_DATA
    Unknown = -1,
    Video(VideoMetadata),
    Audio(AudioMetadata),
    /// Opaque data information usually continuous
    Data,
    Subtitle,
    /// Opaque data information usually sparse
    Attachment,
    Nb,
}