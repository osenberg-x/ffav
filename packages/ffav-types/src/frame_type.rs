use strum_macros::{Display, EnumString};

#[derive(Debug, Display, EnumString, Clone, PartialEq, Eq)]
pub enum FrameType {
	Unknown = -1,
	#[strum(serialize = "I", to_string = "I")]
	I,
	#[strum(serialize = "P", to_string = "P")]
	P,
	#[strum(serialize = "B", to_string = "B")]
	B,
	#[strum(serialize = "Skip", to_string = "Skip")]
	SKIP,
	#[strum(serialize = "Other", to_string = "Other")]
	OTHER,
}