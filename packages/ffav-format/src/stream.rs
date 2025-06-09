use ffav_codec::codec_id::CodecID;
use ffav_codec::codec_par::CodecParameters;

#[derive(Debug)]
struct Stream {
    index: u32,
    codec_id: CodecID,
    codec_tag: u32,
    codec_parameters: CodecParameters,
}