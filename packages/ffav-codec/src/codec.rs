use crate::codec_id::CodecID;
use ffav_types::media_kind::MediaKind;

#[derive(Debug)]
pub struct Codec {
	name: String,
	long_name: String,
	id: CodecID,
	kind: MediaKind,
}