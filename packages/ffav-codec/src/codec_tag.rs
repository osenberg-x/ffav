#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CodecTag(u32);

impl From<u32> for CodecTag {
    fn from(value: u32) -> Self {
        CodecTag(value)
    }
}

impl From<CodecTag> for u32 {
    fn from(value: CodecTag) -> Self {
        value.0
    }
}
