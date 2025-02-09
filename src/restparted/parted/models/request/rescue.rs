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
pub struct RescueRequest {
	pub device: Device,
	pub start: f64,
	pub end: f64,
}

impl From<RescueRequest> for Request {
	fn from(item: RescueRequest) -> Self {
		super::Request {
			command: Command::Rescue,
			device: item.device,
			arguments: vec![item.start.to_string(), item.end.to_string()],
		}
	}
}

impl Deserializable for RescueRequest {
	type Error = RawError;

	fn from_json(data: serde_json::Value) -> Result<Self, Self::Error> {
		let data_device = &data["device"];
		let data_start = &data["start"];
		let data_end = &data["end"];

		if !data_device.is_string() {
			return Err(InvalidJSONError::new(&data_device.to_string()));
		}

		if !data_start.is_f64() {
			return Err(InvalidJSONError::new(&data_start.to_string()));
		}

		if !data_end.is_f64() {
			return Err(InvalidJSONError::new(&data_end.to_string()));
		}

		Ok(RescueRequest {
			device: Device::try_from(data_device.as_str().unwrap())?,
			start: data_start.as_f64().unwrap(),
			end: data_end.as_f64().unwrap(),
		})
	}
}

impl Runable for RescueRequest {}
