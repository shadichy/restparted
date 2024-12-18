use std::error::Error;

use crate::restparted::{
	model::base::Deserializable,
	parted::{
		models::{commands::Command, device::Device},
		system::device::partition_tables::PartitionTable,
	},
};

use super::Request;

pub struct CreateTableRequest {
	pub device: Device,
	pub partition_table: PartitionTable,
}

impl From<CreateTableRequest> for Request {
	fn from(item: CreateTableRequest) -> Self {
		super::Request {
			command: Command::CreateTable,
			device: item.device,
			arguments: vec![item.partition_table.to_string()],
		}
	}
}

impl Deserializable for CreateTableRequest {
	type Error = Box<dyn Error>;

	fn from_json(data: serde_json::Value) -> Result<Self, Self::Error> {
		let data_device = &data["device"];
		let data_partition_table = &data["partition_table"];

		assert!(data_partition_table.is_string() && data_device.is_string());

		Ok(CreateTableRequest {
			device: Device::try_from(data_device.as_str().unwrap())?,
			partition_table: PartitionTable::try_from(data_partition_table.as_str().unwrap())?,
		})
	}
}
