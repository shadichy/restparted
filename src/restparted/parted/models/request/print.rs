use std::error::Error;

use super::Request;
use crate::restparted::{
	model::base::{Deserializable, RawError},
	parted::models::{commands::Command, device::Device},
};

#[derive(PartialEq, Eq)]
pub enum PrintArgument {
	Devices = 0,
	Free = 1,
	All = 2,
}

impl PrintArgument {
	const STR_DEVICES: &'static str = "devices";
	const STR_FREE: &'static str = "free";
	const STR_ALL: &'static str = "all";
}

impl ToString for PrintArgument {
	fn to_string(&self) -> String {
		String::from(match self {
			Self::Devices => Self::STR_DEVICES,
			Self::Free => Self::STR_FREE,
			Self::All => Self::STR_ALL,
		})
	}
}

impl TryFrom<&str> for PrintArgument {
	type Error = Box<dyn Error>;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		match value {
			Self::STR_DEVICES => Ok(Self::Devices),
			Self::STR_FREE => Ok(Self::Free),
			Self::STR_ALL => Ok(Self::All),
			_ => Err(Box::new(RawError::new(value, "Cannot convert"))),
		}
	}
}

impl TryFrom<String> for PrintArgument {
	type Error = Box<dyn Error>;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		PrintArgument::try_from(value.as_str())
	}
}

pub struct PrintRequest {
	pub device: Device,
	pub argument: Option<PrintArgument>,
}

impl PrintRequest {
	fn new(device: Device) -> PrintRequest {
		PrintRequest {
			device: device,
			argument: None,
		}
	}

	fn all() -> PrintRequest {
		PrintRequest {
			device: Device::default(),
			argument: Some(PrintArgument::All),
		}
	}
}

impl From<PrintRequest> for Request {
	fn from(item: PrintRequest) -> Self {
		let args: Vec<String>;
		if item.argument.is_none() {
			args = Vec::new()
		} else {
			args = vec![item.argument.unwrap().to_string()]
		}
		super::Request {
			device: item.device,
			command: Command::Print,
			arguments: args,
		}
	}
}

impl Deserializable for PrintRequest {
	type Error = Box<dyn Error>;

	fn from_json(data: serde_json::Value) -> Result<Self, Self::Error> {
		let data_device = &data["device"];
		let data_argument = &data["argument"];

		if !data_argument.is_string() && !data_device.is_string() {
			return Err(Box::new(RawError::new(
				&data.to_string(),
				"Property does not match type",
			)));
		}

		if data_argument.is_string() && data_argument.as_str().unwrap() == "all" {
			return Ok(PrintRequest::all());
		}

		if data_device.is_string() {
			return Ok(PrintRequest::new(Device::try_from(
				data_device.as_str().unwrap(),
			)?));
		}
		Err(Box::new(RawError::new(
			data.as_str().unwrap(),
			"Failed to parse request",
		)))
	}
}
