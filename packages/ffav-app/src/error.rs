use ffav_types::DataReaderError;
use ffav_types::DataWriterError;
use ffav_demux::DemuxError;
use ffav_mux::MuxError;
use ffav_decode::DecodeError;
use ffav_encode::EncodeError;

pub enum AppError {
	DataReaderError(DataReaderError),
	DataWriterError(DataWriterError),
	DemuxError(DemuxError),
	MuxError(MuxError),
	DecodeError(DecodeError),
	EncodeError(EncodeError),
}

impl From<DataReaderError> for AppError {
	fn from(e: DataReaderError) -> Self {
		Self::DataReaderError(e)
	}
}

impl From<DataWriterError> for AppError {
	fn from(e: DataWriterError) -> Self {
		Self::DataWriterError(e)
	}
}

impl From<DemuxError> for AppError {
	fn from(e: DemuxError) -> Self {
		Self::DemuxError(e)
	}
}

impl From<MuxError> for AppError {
	fn from(e: MuxError) -> Self {
		Self::MuxError(e)
	}
}

impl From<DecodeError> for AppError {
	fn from(e: DecodeError) -> Self {
		Self::DecodeError(e)
	}
}

impl From<EncodeError> for AppError {
	fn from(e: EncodeError) -> Self {
		Self::EncodeError(e)
	}
}