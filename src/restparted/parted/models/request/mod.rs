use std::{error::Error, fmt::Debug};

use align_check::AlignCheckRequest;
use create_part::CreatePartRequest;
use create_table::CreateTableRequest;
use delete_part::DeletePartRequest;
use help::HelpRequest;
use name::NameRequest;
use print::PrintRequest;
use rescue::RescueRequest;
use resize_part::ResizePartRequest;
use serde_json::Value;
use set_flag::SetFlagRequest;
use set_part_flag::SetPartFlagRequest;
use set_type::SetTypeRequest;
use toggle_flag::ToggleFlagRequest;
use toggle_part_flag::TogglePartFlagRequest;
use version::VersionRequest;

use crate::restparted::{
	model::{
		base::serialize::Deserializable,
		errors::{invalid_json::InvalidJSONError, ToRawError},
	},
	parted::{
		command::parted_cmd,
		models::{commands::Command, device::Device},
	},
};

use super::response::Response;

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
		let mut cmd = vec![self.device.to_string(), self.command.get_real_cmd()];
		cmd.extend(self.arguments.clone());
		cmd
	}

	fn try_run<T: Runable, E: Error>(runner: Result<T, E>) -> Response {
		if runner.is_err() {
			Response::from(runner.unwrap_err().to_string().as_str())
		} else {
			runner.unwrap().run()
		}
	}

	pub fn run(data: Value) -> Response {
		let data_command = &data["command"];

		if !data_command.is_string() {
			return Response::new_error(InvalidJSONError::new(&data_command.to_string()));
		}

		match Command::from(data_command.as_str().unwrap()) {
			Command::Version => Self::try_run(VersionRequest::from_json(data.clone())),
			Command::Print => Self::try_run(PrintRequest::from_json(data.clone())),
			Command::AlignCheck => Self::try_run(AlignCheckRequest::from_json(data.clone())),
			Command::Name => Self::try_run(NameRequest::from_json(data.clone())),
			Command::CreateTable => Self::try_run(CreateTableRequest::from_json(data.clone())),
			Command::CreatePart => Self::try_run(CreatePartRequest::from_json(data.clone())),
			Command::Rescue => Self::try_run(RescueRequest::from_json(data.clone())),
			Command::ResizePart => Self::try_run(ResizePartRequest::from_json(data.clone())),
			Command::DeletePart => Self::try_run(DeletePartRequest::from_json(data.clone())),
			Command::SetFlag => Self::try_run(SetFlagRequest::from_json(data.clone())),
			Command::SetPartFlag => Self::try_run(SetPartFlagRequest::from_json(data.clone())),
			Command::ToggleFlag => Self::try_run(ToggleFlagRequest::from_json(data.clone())),
			Command::TogglePartFlag => Self::try_run(TogglePartFlagRequest::from_json(data.clone())),
			Command::SetType => Self::try_run(SetTypeRequest::from_json(data.clone())),
			Command::Help => Self::try_run(HelpRequest::from_json(data.clone())),
		}
	}
}

impl ToString for Request {
	fn to_string(&self) -> String {
		self.to_shell_cmd().join(" ")
	}
}

pub trait Runable : Debug + Clone + Into<Request>{
	fn run(&self) -> Response {
		parted_cmd(Into::<Request>::into((*self).clone()).to_shell_cmd())
	}
}
