use std::error::Error;

use crate::restparted::model::base::UUID;

#[derive(PartialEq, Eq)]
pub enum PartitionID {
	ID(u8),
	UUID(UUID),
}

impl ToString for PartitionID {
	fn to_string(&self) -> String {
		match self {
			PartitionID::ID(val) => format!("{val:#x}"),
			PartitionID::UUID(val) => val.to_string(),
		}
	}
}

impl TryFrom<u8> for PartitionID {
	type Error = Box<dyn Error>;

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		Ok(PartitionID::ID(value))
	}
}

impl TryFrom<i8> for PartitionID {
	type Error = Box<dyn Error>;

	fn try_from(value: i8) -> Result<Self, Self::Error> {
		Ok(PartitionID::ID(value as u8))
	}
}

impl TryFrom<&str> for PartitionID {
	type Error = Box<dyn Error>;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		Ok(if value.len() <= 3 || (value.len() == 4 && value.starts_with("0x")) {
      PartitionID::ID(u8::from_str_radix(value, 16)?)
		} else {
			PartitionID::UUID(UUID::try_from(value)?)
		})
	}
}

impl TryFrom<String> for PartitionID {
	type Error = Box<dyn Error>;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		PartitionID::try_from(value.as_str())
	}
}

impl TryFrom<u128> for PartitionID {
	type Error = Box<dyn Error>;

	fn try_from(value: u128) -> Result<Self, Self::Error> {
		Ok(PartitionID::UUID(UUID::try_from(value)?))
	}
}

impl TryFrom<i128> for PartitionID {
	type Error = Box<dyn Error>;

	fn try_from(value: i128) -> Result<Self, Self::Error> {
		Ok(PartitionID::UUID(UUID::try_from(value)?))
	}
}
