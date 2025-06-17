use strum_macros::{Display, EnumString};

#[derive(Debug, Display, EnumString, Clone, PartialEq, Eq)]
pub enum FrameType {
	#[strum(serialize = "Unknown")]
	Unknown = -1,
	#[strum(serialize = "I")]
	I,
	#[strum(serialize = "P")]
	P,
	#[strum(serialize = "B")]
	B,
	#[strum(serialize = "Skip")]
	SKIP,
	#[strum(serialize = "Other")]
	OTHER,
}