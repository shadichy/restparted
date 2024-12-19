use crate::restparted::model::errors::{enum_conversion::EnumConversionError, RawError, ToRawError};

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum FlagState {
	Off = 0,
	On = 1,
}

impl FlagState {
	const STR_OFF: &'static str = "off";
	const STR_ON: &'static str = "on";
}

impl ToString for FlagState {
	fn to_string(&self) -> String {
		String::from(match self {
			Self::Off => Self::STR_OFF,
			Self::On => Self::STR_ON,
		})
	}
}

impl TryFrom<&str> for FlagState {
	type Error = RawError;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		match value {
			Self::STR_OFF => Ok(Self::Off),
			Self::STR_ON => Ok(Self::On),
			_ => Err(EnumConversionError::new(value)),
		}
	}
}

impl TryFrom<String> for FlagState {
	type Error = RawError;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		Self::try_from(value.as_str())
	}
}

impl TryFrom<usize> for FlagState {
	type Error = RawError;

	fn try_from(value: usize) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::Off),
			1 => Ok(Self::On),
			_ => Err(EnumConversionError::new(&value.to_string())),
		}
	}
}
