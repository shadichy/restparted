use crate::restparted::{
	model::{
		base::serialize::Deserializable,
		errors::{invalid_json::InvalidJSONError, RawError, ToRawError},
	},
	parted::{
		models::{
			commands::Command,
			device::Device,
			request::{Request, Runable},
		},
		system::device::partition_tables::PartitionTable,
	},
};

#[derive(Clone, Debug)]
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
	type Error = RawError;

	fn from_json(data: serde_json::Value) -> Result<Self, Self::Error> {
		let data_device = &data["device"];
		let data_partition_table = &data["partition_table"];

		if !data_device.is_string() {
			return Err(InvalidJSONError::new(&data_device.to_string()));
		}

		if !data_partition_table.is_string() {
			return Err(InvalidJSONError::new(&data_partition_table.to_string()));
		}

		Ok(CreateTableRequest {
			device: Device::try_from(data_device.as_str().unwrap())?,
			partition_table: PartitionTable::try_from(data_partition_table.as_str().unwrap())?,
		})
	}
}

impl Runable for CreateTableRequest {}
