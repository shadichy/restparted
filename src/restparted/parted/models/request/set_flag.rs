use std::error::Error;

use crate::restparted::{
	model::base::Deserializable,
	parted::{
		models::{commands::Command, device::Device},
		system::device::{disk_flags::DiskFlag, flag_state::FlagState},
	},
};

use super::Request;

pub struct SetFlagRequest {
	pub device: Device,
	pub flag: DiskFlag,
	pub state: FlagState,
}

impl From<SetFlagRequest> for Request {
	fn from(item: SetFlagRequest) -> Self {
		Request {
			command: Command::SetFlag,
			device: item.device,
			arguments: vec![item.flag.to_string(), item.state.to_string()],
		}
	}
}

impl Deserializable for SetFlagRequest {
	type Error = Box<dyn Error>;

	fn from_json(data: serde_json::Value) -> Result<Self, Self::Error> {
		let data_device = &data["device"];
		let data_flag = &data["flag"];
		let data_state = &data["state"];
		assert!(
			data_device.is_string()
				&& data_flag.is_string()
				&& (data_state.is_string() || data_state.is_u64())
		);
		let state: FlagState;
		if data_state.is_string() {
			state = FlagState::try_from(data_state.as_str().unwrap())?
		} else {
			state = FlagState::try_from(data_state.as_f64().unwrap() as usize)?
		}
		Ok(SetFlagRequest {
			device: Device::try_from(data_device.as_str().unwrap())?,
			flag: DiskFlag::try_from(data_flag.as_str().unwrap())?,
			state,
		})
	}
}
