use std::str::FromStr;
use std::string::ToString;
use strum_macros::{Display, EnumString};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumString, Display)]
pub enum MediaKind {
    /// Usually treated as AVMEDIA_TYPE_DATA
    Unknown = -1,
    #[strum(serialize = "Video", to_string = "video")]
    Video,
    #[strum(serialize = "Audio", to_string = "audio")]
    Audio,
    /// Opaque data information usually continuous
    #[strum(serialize = "Data", to_string = "data")]
    Data,
    #[strum(serialize = "Subtitle", to_string = "subtitle")]
    Subtitle,
    /// Opaque data information usually sparse
    #[strum(serialize = "Attachment", to_string = "attachment")]
    Attachment,
    Nb,
}