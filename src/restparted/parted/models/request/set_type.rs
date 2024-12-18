use std::error::Error;

use crate::restparted::{
	model::base::Deserializable,
	parted::{
		models::{commands::Command, device::Device},
		system::device::partition_id::PartitionID,
	},
};

use super::Request;

pub struct SetTypeRequest {
	pub device: Device,
	pub partition_number: u64,
	pub type_id: PartitionID,
}

impl From<SetTypeRequest> for Request {
	fn from(item: SetTypeRequest) -> Self {
		super::Request {
			command: Command::Rescue,
			device: item.device,
			arguments: vec![item.partition_number.to_string(), item.type_id.to_string()],
		}
	}
}

impl Deserializable for SetTypeRequest {
	type Error = Box<dyn Error>;

	fn from_json(data: serde_json::Value) -> Result<Self, Self::Error> {
		let data_device = &data["device"];
		let data_partition_number = &data["number"];
		let data_type_id = &data["id"];
		assert!(
			data_device.is_string()
				&& data_partition_number.is_u64()
				&& (data_type_id.is_string() || data_type_id.is_i64())
		);
    let type_id: PartitionID;
    if data_type_id.is_string() {
      type_id = PartitionID::try_from(data_type_id.as_str().unwrap())?
    } else {
      type_id = PartitionID::try_from(data_type_id.as_u64().unwrap() as u8)?
    }
		Ok(SetTypeRequest {
			device: Device::try_from(data_device.as_str().unwrap())?,
			partition_number: data_partition_number.as_u64().unwrap() as u64,
			type_id,
		})
	}
}
