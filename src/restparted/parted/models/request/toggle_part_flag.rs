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
		system::device::partition_flags::PartitionFlag,
	},
};

#[derive(Clone, Debug)]
pub struct TogglePartFlagRequest {
	pub device: Device,
	pub partition_number: Option<u64>,
	pub flag: Option<PartitionFlag>,
}

impl From<TogglePartFlagRequest> for Request {
	fn from(item: TogglePartFlagRequest) -> Self {
		let mut args: Vec<String> = Vec::new();
		if item.partition_number.is_some() {
			args.push(item.partition_number.unwrap().to_string());
			if item.flag.is_some() {
				args.push(item.flag.unwrap().to_string())
			}
		}
		Request {
			command: Command::TogglePartFlag,
			device: item.device,
			arguments: args,
		}
	}
}

impl Deserializable for TogglePartFlagRequest {
	type Error = RawError;

	fn from_json(data: serde_json::Value) -> Result<Self, Self::Error> {
		let data_device = &data["device"];
		let data_partition_number = &data["number"];
		let data_flag = &data["flag"];

		if !data_device.is_string() {
			return Err(InvalidJSONError::new(&data_device.to_string()));
		}

		let flag: Option<PartitionFlag>;
		if data_flag.is_string() {
			flag = Some(PartitionFlag::try_from(data_flag.as_str().unwrap())?)
		} else {
			flag = None
		}
		Ok(TogglePartFlagRequest {
			device: Device::try_from(data_device.as_str().unwrap())?,
			partition_number: data_partition_number.as_i64().map(|n| n as u64),
			flag,
		})
	}
}

impl Runable for TogglePartFlagRequest {}
