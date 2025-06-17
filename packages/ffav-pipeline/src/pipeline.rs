use ffav_demux::Demuxer;
use ffav_mux::Muxer;
use ffav_filter::Filter;
use ffav_decode::Decoder;
use ffav_encode::Encoder;

pub struct Pipeline {
	demuxer: Option<Box<dyn Demuxer>>,
	muxer: Option<Box<dyn Muxer>>,
	filter: Option<Box<dyn Filter>>,
	decoder: Option<Box<dyn Decoder>>,
	encoder: Option<Box<dyn Encoder>>,
}

impl Pipeline {
	pub fn new() -> Self {
		Self {
			demuxer: None,
			muxer: None,
			filter: None,
			decoder: None,
			encoder: None,
		}
	}
}