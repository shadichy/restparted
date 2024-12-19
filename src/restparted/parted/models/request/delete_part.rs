use crate::restparted::{
	model::{
		base::serialize::Deserializable,
		errors::{invalid_json::InvalidJSONError, RawError, ToRawError},
	},
	parted::models::{
		commands::Command,
		device::Device,
		request::{Request, Runable},
	},
};

#[derive(Clone, Debug)]
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
	type Error = RawError;

	fn from_json(data: serde_json::Value) -> Result<Self, Self::Error> {
		let data_device = &data["device"];
		let data_partition_number = &data["number"];

		if !data_device.is_string() {
			return Err(InvalidJSONError::new(&data_device.to_string()));
		}

		if !data_partition_number.is_u64() {
			return Err(InvalidJSONError::new(&data_partition_number.to_string()));
		}

		Ok(DeletePartRequest {
			device: Device::try_from(data_device.as_str().unwrap())?,
			partition_number: data_partition_number.as_u64().unwrap() as u16,
		})
	}
}
impl Runable for DeletePartRequest {}
