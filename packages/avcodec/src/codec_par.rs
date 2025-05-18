use avutil::util::MediaType;
use crate::codec_id::CodecID;
use crate::codec_tag::CodecTag;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodecExtraParameters(Option<Vec<u8>>);

impl From<Vec<u8>> for CodecExtraParameters {
    fn from(data: Vec<u8>) -> Self {
        CodecExtraParameters(Some(data))
    }
}

impl Into<Option<Vec<u8>>> for CodecExtraParameters {
    fn into(self) -> Option<Vec<u8>> {
        self.0
    }
}

pub struct PacketSideData {
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodecParameters {
    /**
     * General type of the encoded data.
     */
    codec_type: MediaType,
    /**
     * Specific type of the encoded data (the codec used).
     */
    codec_id: CodecID,

    /**
     * Additional information about the codec (corresponds to the AVI FOURCC).
     */
    codec_tag: CodecTag,

    /**
     * Extra binary data needed for initializing the decoder, codec-dependent.
     */
    extra_parameters: CodecExtraParameters,
}
