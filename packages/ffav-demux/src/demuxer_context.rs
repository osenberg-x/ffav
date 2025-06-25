use ffav_utils::DataReader;

// #[derive(Debug)]
pub struct DemuxerContext {
	reader: Box<dyn DataReader>,
}