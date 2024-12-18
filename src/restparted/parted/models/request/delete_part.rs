use std::error::Error;

use crate::restparted::{
	model::base::Deserializable,
	parted::{models::commands::Command, models::device::Device},
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

		assert!(data_partition_number.is_u64() && data_device.is_string());

		Ok(DeletePartRequest {
			device: Device::try_from(data_device.as_str().unwrap())?,
			partition_number: data_partition_number.as_u64().unwrap() as u16,
		})
	}
}
