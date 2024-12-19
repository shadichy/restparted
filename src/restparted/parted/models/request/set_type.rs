use crate::restparted::{
	model::{base::serialize::Deserializable, errors::{invalid_json::InvalidJSONError, RawError, ToRawError}},
	parted::{
		models::{
			commands::Command,
			device::Device,
			request::{Request, Runable},
		},
		system::device::partition_id::PartitionID,
	},
};

#[derive(Clone, Debug)]
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
	type Error = RawError;

	fn from_json(data: serde_json::Value) -> Result<Self, Self::Error> {
		let data_device = &data["device"];
		let data_partition_number = &data["number"];
		let data_type_id = &data["id"];

		if !data_device.is_string() {
			return Err(InvalidJSONError::new(&data_device.to_string()));
		}

		if !data_partition_number.is_u64() {
			return Err(InvalidJSONError::new(&data_partition_number.to_string()));
		}

		if !data_type_id.is_i64() && !data_type_id.is_string() {
			return Err(InvalidJSONError::new(&data_type_id.to_string()));
		}

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

impl Runable for SetTypeRequest {}
