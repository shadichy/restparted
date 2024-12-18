use std::error::Error;

use crate::restparted::{
	model::base::{Deserializable, RawError},
	parted::models::{commands::Command, device::Device},
};

use super::Request;

pub struct DeletePartRequest {
	pub device: Device,
	pub partition_number: u16,
}

impl From<DeletePartRequest> for Request {
	fn from(item: DeletePartRequest) -> Self {
		super::Request {
			command: Command::DeletePart,
			device: item.device,
			arguments: vec![item.partition_number.to_string()],
		}
	}
}

impl Deserializable for DeletePartRequest {
	type Error = Box<dyn Error>;

	fn from_json(data: serde_json::Value) -> Result<Self, Self::Error> {
		let data_device = &data["device"];
		let data_partition_number = &data["number"];

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

		Ok(DeletePartRequest {
			device: Device::try_from(data_device.as_str().unwrap())?,
			partition_number: data_partition_number.as_u64().unwrap() as u16,
		})
	}
}
