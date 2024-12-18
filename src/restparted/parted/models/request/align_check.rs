use std::error::Error;

use crate::restparted::{
	model::base::{Deserializable, RawError},
	parted::models::{commands::Command, device::Device},
};

use super::Request;

#[derive(PartialEq, Eq)]
pub enum AlignCheckType {
	Minimal = 0,
	Optimal = 1,
}

impl AlignCheckType {
	const STR_MINIMAL: &'static str = "minimal";
	const STR_OPTIMAL: &'static str = "optimal";
}

impl ToString for AlignCheckType {
	fn to_string(&self) -> String {
		String::from(match self {
			Self::Minimal => Self::STR_MINIMAL,
			Self::Optimal => Self::STR_OPTIMAL,
		})
	}
}

impl TryFrom<&str> for AlignCheckType {
	type Error = Box<dyn Error>;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		match value {
			Self::STR_MINIMAL => Ok(Self::Minimal),
			Self::STR_OPTIMAL => Ok(Self::Optimal),
			_ => Err(Box::new(RawError::new(value, "Cannot convert"))),
		}
	}
}

impl TryFrom<String> for AlignCheckType {
	type Error = Box<dyn Error>;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		Self::try_from(value.as_str())
	}
}

pub struct AlignCheckRequest {
	pub device: Device,
	pub align_check_type: AlignCheckType,
	pub partition_number: u16,
}

impl From<AlignCheckRequest> for Request {
	fn from(item: AlignCheckRequest) -> Self {
		Request {
			command: Command::AlignCheck,
			device: item.device,
			arguments: vec![
				item.align_check_type.to_string(),
				item.partition_number.to_string(),
			],
		}
	}
}

impl Deserializable for AlignCheckRequest {
	type Error = Box<dyn Error>;

	fn from_json(data: serde_json::Value) -> Result<Self, Self::Error> {
		let data_device = &data["device"];
		let data_align_check_type = &data["type"];
		let data_partition_number = &data["number"];

		if !data_device.is_string() {
			return Err(Box::new(RawError::new(
				&data_device.to_string(),
				"Property does not match type",
			)));
		}

		if !data_align_check_type.is_string() {
			return Err(Box::new(RawError::new(
				&data_align_check_type.to_string(),
				"Property does not match type",
			)));
		}

		if !data_partition_number.is_u64() {
			return Err(Box::new(RawError::new(
				&data_partition_number.to_string(),
				"Property does not match type",
			)));
		}

		Ok(AlignCheckRequest {
			device: Device::try_from(data_device.as_str().unwrap())?,
			align_check_type: AlignCheckType::try_from(data_align_check_type.as_str().unwrap())?,
			partition_number: data_partition_number.as_u64().unwrap() as u16,
		})
	}
}
