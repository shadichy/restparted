
use regex::Regex;

use crate::restparted::model::errors::{invalid_uuid::InvalidUUIDError, RawError, ToRawError};

#[derive(Clone, Debug)]
pub struct UUID(String);

impl ToString for UUID {
	fn to_string(&self) -> String {
		self.0.clone()
	}
}

impl TryFrom<&str> for UUID {
	type Error = RawError;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		if value.len() == 32 {
			let mut str = String::from(&value[..8]);
			str += "-";
			str += &value[8..12];
			str += "-";
			str += &value[12..16];
			str += "-";
			str += &value[16..20];
			str += "-";
			str += &value[20..];
			Ok(UUID(str))
		} else if value.len() == 36 && Regex::new(r"-")?.find_iter(value).count() == 4 {
			Ok(UUID(String::from(value)))
		} else {
			Err(InvalidUUIDError::new(value))
		}
	}
}

impl TryFrom<String> for UUID {
	type Error = RawError;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		UUID::try_from(value.as_str())
	}
}

impl TryFrom<u128> for UUID {
	type Error = RawError;

	fn try_from(_value: u128) -> Result<Self, Self::Error> {
		// format!("{value:x}");
		todo!()
	}
}

impl TryFrom<i128> for UUID {
	type Error = RawError;

	fn try_from(value: i128) -> Result<Self, Self::Error> {
		UUID::try_from(value as u128)
	}
}

impl PartialEq for UUID {
	fn eq(&self, other: &Self) -> bool {
		self.0 == other.0
	}
}

impl Eq for UUID {}
