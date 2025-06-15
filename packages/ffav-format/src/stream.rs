use ffav_types::codec_id::CodecID;
use ffav_types::codec_parameters::CodecParameters;

#[derive(Debug)]
struct Stream {
    index: u32,
    codec_id: CodecID,
    codec_tag: u32,
    codec_parameters: CodecParameters,
}