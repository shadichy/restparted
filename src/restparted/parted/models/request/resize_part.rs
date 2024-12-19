use crate::restparted::{
	model::{base::serialize::Deserializable, errors::{invalid_json::InvalidJSONError, RawError, ToRawError}},
	parted::models::{
		commands::Command,
		device::Device,
		request::{Request, Runable},
	},
};

#[derive(Clone, Debug)]
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
	type Error = RawError;

	fn from_json(data: serde_json::Value) -> Result<Self, Self::Error> {
		let data_device = &data["device"];
		let data_partition_number = &data["number"];
		let data_end = &data["end"];

		if !data_device.is_string() {
			return Err(InvalidJSONError::new(&data_device.to_string()));
		}

		if !data_partition_number.is_u64() {
			return Err(InvalidJSONError::new(&data_partition_number.to_string()));
		}

		if !data_end.is_f64() {
			return Err(InvalidJSONError::new(&data_end.to_string()));
		}

    Ok(ResizePartRequest {
			device: Device::try_from(data_device.as_str().unwrap())?,
			partition_number: data_partition_number.as_u64().unwrap(),
			end: data_end.as_f64().unwrap(),
		})
	}
}

impl Runable for ResizePartRequest {
    
}
