use crate::restparted::{
	model::{base::serialize::Deserializable, errors::{invalid_json::InvalidJSONError, RawError, ToRawError}},
	parted::{
		models::{
			commands::Command,
			device::Device,
			request::{Request, Runable},
		},
		system::device::{flag_state::FlagState, partition_flags::PartitionFlag},
	},
};

#[derive(Clone, Debug)]
pub struct SetPartFlagRequest {
	pub device: Device,
	pub partition_number: u64,
	pub flag: PartitionFlag,
	pub state: FlagState,
}

impl From<SetPartFlagRequest> for Request {
	fn from(item: SetPartFlagRequest) -> Self {
		Request {
			command: Command::SetPartFlag,
			device: item.device,
			arguments: vec![
				item.partition_number.to_string(),
				item.flag.to_string(),
				item.state.to_string(),
			],
		}
	}
}

impl Deserializable for SetPartFlagRequest {
	type Error = RawError;

	fn from_json(data: serde_json::Value) -> Result<Self, Self::Error> {
		let data_device = &data["device"];
		let data_partition_number = &data["number"];
		let data_flag = &data["flag"];
		let data_state = &data["state"];

		if !data_device.is_string() {
			return Err(InvalidJSONError::new(&data_device.to_string()));
		}

		if !data_partition_number.is_u64() {
			return Err(InvalidJSONError::new(&data_partition_number.to_string()));
		}

		if !data_flag.is_string() {
			return Err(InvalidJSONError::new(&data_flag.to_string()));
		}

		if !data_state.is_u64() && !data_state.is_string() {
			return Err(InvalidJSONError::new(&data_state.to_string()));
		}

		let state: FlagState;
		if data_state.is_string() {
			state = FlagState::try_from(data_state.as_str().unwrap())?
		} else {
			state = FlagState::try_from(data_state.as_u64().unwrap() as usize)?
		}
		Ok(SetPartFlagRequest {
			device: Device::try_from(data_device.as_str().unwrap())?,
			partition_number: data_partition_number.as_i64().unwrap() as u64,
			flag: PartitionFlag::try_from(data_flag.as_str().unwrap())?,
			state,
		})
	}
}

impl Runable for SetPartFlagRequest {}
