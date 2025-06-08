use ffav_codec::codec_id::CodecID;

#[derive(Debug)]
struct Stream {
    index: u32,
    codec_id: CodecID,
    codec_tag: u32,
}