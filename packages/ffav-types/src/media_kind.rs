use crate::{audio_attribute::AudioAttribute, video_attribute::VideoAttribute};


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum MediaKind {
    /// Usually treated as AVMEDIA_TYPE_DATA
    Unknown = -1,
    Video(VideoAttribute),
    Audio(AudioAttribute),
    /// Opaque data information usually continuous
    Data,
    Subtitle,
    /// Opaque data information usually sparse
    Attachment,
    Nb,
}