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
pub struct MovePartRequest {
	pub device: Device,
	pub partition_number: u64,
	pub start: f64,
}

impl From<MovePartRequest> for Request {
	fn from(item: MovePartRequest) -> Self {
		super::Request {
			command: Command::Rescue,
			device: item.device,
			arguments: vec![],
		}
	}
}

impl Deserializable for MovePartRequest {
	type Error = RawError;

	fn from_json(data: serde_json::Value) -> Result<Self, Self::Error> {
		let data_device = &data["device"];
		let data_partition_number = &data["number"];
		let data_start = &data["start"];

		if !data_device.is_string() {
			return Err(InvalidJSONError::new(&data_device.to_string()));
		}

		if !data_partition_number.is_u64() {
			return Err(InvalidJSONError::new(&data_partition_number.to_string()));
		}

		if !data_start.is_f64() {
			return Err(InvalidJSONError::new(&data_start.to_string()));
		}

		Ok(MovePartRequest {
			device: Device::try_from(data_device.as_str().unwrap())?,
			partition_number: data_partition_number.as_u64().unwrap(),
			start: data_start.as_f64().unwrap(),
		})
	}
}

impl Runable for MovePartRequest {
	fn run(&self) -> crate::restparted::parted::models::response::Response {
		todo!()
	}
}
