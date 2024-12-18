use std::error::Error;

use crate::restparted::{
	model::base::{Deserializable, RawError},
	parted::{
		models::{commands::Command, device::Device},
		system::device::{flag_state::FlagState, partition_flags::PartitionFlag},
	},
};

use super::Request;

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
	type Error = Box<dyn Error>;

	fn from_json(data: serde_json::Value) -> Result<Self, Self::Error> {
		let data_device = &data["device"];
		let data_partition_number = &data["number"];
		let data_flag = &data["flag"];
		let data_state = &data["state"];

		if !data_device.is_string() {
			return Err(Box::new(RawError::new(
				&data_device.to_string(),
				"Property does not match type",
			)));
		}

		if !data_partition_number.is_u64() {
			return Err(Box::new(RawError::new(
				&data_partition_number.to_string(),
				"Property does not match type",
			)));
		}

		if !data_flag.is_string() {
			return Err(Box::new(RawError::new(
				&data_flag.to_string(),
				"Property does not match type",
			)));
		}

		if !data_state.is_u64() && !data_state.is_string() {
			return Err(Box::new(RawError::new(
				&data_state.to_string(),
				"Property does not match type",
			)));
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
