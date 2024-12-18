use std::error::Error;

use crate::restparted::{
	model::base::Deserializable,
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
		assert!(data_device.is_string() && data_partition_number.is_u64() && data_end.is_f64());
    Ok(ResizePartRequest {
			device: Device::try_from(data_device.as_str().unwrap())?,
			partition_number: data_partition_number.as_u64().unwrap(),
			end: data_end.as_f64().unwrap(),
		})
	}
}
