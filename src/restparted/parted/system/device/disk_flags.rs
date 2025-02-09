use crate::restparted::model::errors::{
	enum_conversion::EnumConversionError, RawError, ToRawError,
};

#[allow(non_camel_case_types)]
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum DiskFlag {
	CYLINDER_ALIGNMENT = 0,
	PMBR_BOOT = 1,
}

impl DiskFlag {
	const STR_CYLINDER_ALIGNMENT: &'static str = "cylinder_alignment";
	const STR_PMBR_BOOT: &'static str = "pmbr_boot";
}

impl ToString for DiskFlag {
	fn to_string(&self) -> String {
		String::from(match self {
			Self::CYLINDER_ALIGNMENT => Self::STR_CYLINDER_ALIGNMENT,
			Self::PMBR_BOOT => Self::STR_PMBR_BOOT,
		})
	}
}

impl TryFrom<&str> for DiskFlag {
	type Error = RawError;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		match value {
			Self::STR_CYLINDER_ALIGNMENT => Ok(Self::CYLINDER_ALIGNMENT),
			Self::STR_PMBR_BOOT => Ok(Self::PMBR_BOOT),
			_ => Err(EnumConversionError::new(value)),
		}
	}
}

impl TryFrom<String> for DiskFlag {
	type Error = RawError;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		Self::try_from(value.as_str())
	}
}
