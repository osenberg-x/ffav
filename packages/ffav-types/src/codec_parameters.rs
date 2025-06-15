use crate::media_kind::MediaKind;
use crate::codec_id::CodecID;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodecParameters {
    /**
     * General type of the encoded data.
     */
    codec_type: MediaKind,
    /**
     * Specific type of the encoded data (the codec used).
     */
    codec_id: CodecID,

    // /**
    //  * Extra binary data needed for initializing the decoder, codec-dependent.
    //  */
    // extra_parameters: CodecExtraParameters,
}