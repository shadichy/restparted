use std::error::Error;

use crate::restparted::{
	model::base::{Deserializable, RawError},
	parted::models::{commands::Command, device::Device},
};

use super::Request;

pub struct ResizePartRequest {
	pub device: Device,
	pub partition_number: u64,
	pub end: f64,
}

impl From<ResizePartRequest> for Request {
	fn from(item: ResizePartRequest) -> Self {
		super::Request {
			command: Command::Rescue,
			device: item.device,
			arguments: vec![item.partition_number.to_string(), item.end.to_string()],
		}
	}
}

impl Deserializable for ResizePartRequest {
	type Error = Box<dyn Error>;

	fn from_json(data: serde_json::Value) -> Result<Self, Self::Error> {
		let data_device = &data["device"];
		let data_partition_number = &data["number"];
		let data_end = &data["end"];

		if !data_device.is_string() {
			return Err(Box::new(RawError::new(
				&data_device.to_string(),
				"Property does not match type",
			)));
		}

		if !data_partition_number.is_u64() {
			return Err(Box::new(RawError::new(
				&data_partition_number.to_string(),
				"Property does not match type",
			)));
		}

		if !data_end.is_f64() {
			return Err(Box::new(RawError::new(
				&data_end.to_string(),
				"Property does not match type",
			)));
		}

    Ok(ResizePartRequest {
			device: Device::try_from(data_device.as_str().unwrap())?,
			partition_number: data_partition_number.as_u64().unwrap(),
			end: data_end.as_f64().unwrap(),
		})
	}
}
