use std::error::Error;

use crate::restparted::{model::base::Deserializable, parted::models::commands::Command};

use super::Request;

pub struct HelpRequest {
	pub target: Command,
}

impl From<HelpRequest> for Request {
	fn from(item: HelpRequest) -> Self {
		if item.target == Command::Help {
			super::Request::new()
		} else {
			super::Request::new_cmd_arg(Command::Help, vec![item.target.to_string()])
		}
	}
}

impl Deserializable for HelpRequest {
	type Error = Box<dyn Error>;

	fn from_json(data: serde_json::Value) -> Result<Self, Self::Error> {
		let data_target = &data["target"];

		let real_target: Command;
		if data_target.is_string() {
			real_target = Command::from(data_target.as_str().unwrap());
		} else {
			real_target = Command::Help;
		}

		Ok(HelpRequest {
			target: real_target,
		})
	}
}
