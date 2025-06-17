use crate::codec_id::CodecID;
use crate::rational::Rational;

#[derive(Debug)]
pub struct StreamAttribute {
    pub index: usize,
    pub codec_id: CodecID,
	pub time_base: Rational,
	pub duration: i64,
	pub bit_rate: i64,
}
