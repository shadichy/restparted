use std::error::Error;

use crate::restparted::{
	model::base::{Deserializable, RawError},
	parted::models::{commands::Command, device::Device},
};

use super::Request;

pub struct NameRequest {
	pub device: Device,
	pub label: String,
}

impl From<NameRequest> for Request {
	fn from(item: NameRequest) -> Self {
		super::Request {
			command: Command::Name,
			device: item.device,
			arguments: vec![item.label],
		}
	}
}

impl Deserializable for NameRequest {
	type Error = Box<dyn Error>;

	fn from_json(data: serde_json::Value) -> Result<Self, Self::Error> {
		let data_device = &data["device"];
		let data_label = &data["label"];

		if !data_device.is_string() {
			return Err(Box::new(RawError::new(
				&data_device.to_string(),
				"Property does not match type",
			)));
		}

		if !data_label.is_string() {
			return Err(Box::new(RawError::new(
				&data_label.to_string(),
				"Property does not match type",
			)));
		}

		Ok(NameRequest {
			device: Device::try_from(data_device.as_str().unwrap())?,
			label: String::from(data_label.as_str().unwrap()),
		})
	}
}
