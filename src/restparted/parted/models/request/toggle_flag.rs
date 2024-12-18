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
		system::device::disk_flags::DiskFlag,
	},
};

#[derive(Clone, Debug)]
pub struct ToggleFlagRequest {
	pub device: Device,
	pub flag: Option<DiskFlag>,
}

impl From<ToggleFlagRequest> for Request {
	fn from(item: ToggleFlagRequest) -> Self {
		let args: Vec<String>;
		if item.flag.is_none() {
			args = Vec::new()
		} else {
			args = vec![item.flag.unwrap().to_string()]
		}
		Request {
			command: Command::ToggleFlag,
			device: item.device,
			arguments: args,
		}
	}
}

impl Deserializable for ToggleFlagRequest {
	type Error = RawError;

	fn from_json(data: serde_json::Value) -> Result<Self, Self::Error> {
		let data_device = &data["device"];
		let data_flag = &data["flag"];

		if !data_device.is_string() {
			return Err(InvalidJSONError::new(&data_device.to_string()));
		}

		let flag: Option<DiskFlag>;
		if data_flag.is_string() {
			flag = Some(DiskFlag::try_from(data_flag.as_str().unwrap())?)
		} else {
			flag = None
		}
		Ok(ToggleFlagRequest {
			device: Device::try_from(data_device.as_str().unwrap())?,
			flag,
		})
	}
}

impl Runable for ToggleFlagRequest {}
