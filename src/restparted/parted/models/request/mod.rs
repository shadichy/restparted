use std::error::Error;

use align_check::AlignCheckRequest;
use create_part::CreatePartRequest;
use create_table::CreateTableRequest;
use delete_part::DeletePartRequest;
use help::HelpRequest;
use name::NameRequest;
use print::PrintRequest;
use rescue::RescueRequest;
use resize_part::ResizePartRequest;
use set_flag::SetFlagRequest;
use set_part_flag::SetPartFlagRequest;
use set_type::SetTypeRequest;
use toggle_flag::ToggleFlagRequest;
use toggle_part_flag::TogglePartFlagRequest;
use version::VersionRequest;

use crate::restparted::{
	model::base::{Deserializable, RawError},
	parted::models::{commands::Command, device::Device},
};

mod align_check;
mod create_part;
mod create_table;
mod delete_part;
mod help;
mod name;
mod print;
mod rescue;
mod resize_part;
mod set_flag;
mod set_part_flag;
mod set_type;
mod toggle_flag;
mod toggle_part_flag;
mod version;

pub struct Request {
	pub command: Command,
	pub device: Device,
	pub arguments: Vec<String>,
}

impl Request {
	pub fn new() -> Self {
		Self::new_cmd(Command::new())
	}

	pub fn new_cmd(command: Command) -> Self {
		Self::new_cmd_arg(command, Vec::new())
	}

	pub fn new_cmd_arg(command: Command, arguments: Vec<String>) -> Self {
		Request {
			command: command,
			device: Device::default(),
			arguments: arguments,
		}
	}

	pub fn to_shell_cmd(&self) -> Vec<String> {
		let mut cmd = vec![
			String::from("parted"),
			self.device.path.clone(),
			String::from("--json"),
			self.command.get_real_cmd(),
		];
		cmd.extend(self.arguments.clone());
		cmd
	}
}

impl ToString for Request {
	fn to_string(&self) -> String {
		self.to_shell_cmd().join(" ")
	}
}

impl Deserializable for Request {
	type Error = Box<dyn Error>;

	fn from_json(data: serde_json::Value) -> Result<Self, Self::Error> {
		let data_command = &data["command"];

		if !data_command.is_string() {
			return Err(Box::new(RawError::new(
				&data_command.to_string(),
				"Property does not match type",
			)));
		}

		Ok(match Command::from(data_command.as_str().unwrap()) {
			Command::Version => Self::from(VersionRequest::from_json(data.clone())?),
			Command::Print => Self::from(PrintRequest::from_json(data.clone())?),
			Command::AlignCheck => Self::from(AlignCheckRequest::from_json(data.clone())?),
			Command::Name => Self::from(NameRequest::from_json(data.clone())?),
			Command::CreateTable => Self::from(CreateTableRequest::from_json(data.clone())?),
			Command::CreatePart => Self::from(CreatePartRequest::from_json(data.clone())?),
			Command::Rescue => Self::from(RescueRequest::from_json(data.clone())?),
			Command::ResizePart => Self::from(ResizePartRequest::from_json(data.clone())?),
			Command::DeletePart => Self::from(DeletePartRequest::from_json(data.clone())?),
			Command::SetFlag => Self::from(SetFlagRequest::from_json(data.clone())?),
			Command::SetPartFlag => Self::from(SetPartFlagRequest::from_json(data.clone())?),
			Command::ToggleFlag => Self::from(ToggleFlagRequest::from_json(data.clone())?),
			Command::TogglePartFlag => Self::from(TogglePartFlagRequest::from_json(data.clone())?),
			Command::SetType => Self::from(SetTypeRequest::from_json(data.clone())?),
			Command::Help => Self::from(HelpRequest::from_json(data.clone())?),
		})
	}
}
