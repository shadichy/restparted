use crate::restparted::model::errors::{enum_conversion::EnumConversionError, RawError, ToRawError};

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum PartitionType {
	PRIMARY = 0,
	LOGICAL = 1,
	EXTENDED = 2,
}

impl PartitionType {
	const STR_PRIMARY: &'static str = "primary";
	const STR_LOGICAL: &'static str = "logical";
	const STR_EXTENDED: &'static str = "extended";
}

impl ToString for PartitionType {
	fn to_string(&self) -> String {
		String::from(match self {
			Self::PRIMARY => Self::STR_PRIMARY,
			Self::LOGICAL => Self::STR_LOGICAL,
			Self::EXTENDED => Self::STR_EXTENDED,
		})
	}
}

impl TryFrom<&str> for PartitionType {
	type Error = RawError;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		match value {
			Self::STR_PRIMARY => Ok(Self::PRIMARY),
			Self::STR_LOGICAL => Ok(Self::LOGICAL),
			Self::STR_EXTENDED => Ok(Self::EXTENDED),
			_ => Err(EnumConversionError::new(value)),
		}
	}
}

impl TryFrom<String> for PartitionType {
	type Error = RawError;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		Self::try_from(value.as_str())
	}
}
