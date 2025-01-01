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
		errors::{invalid_json::InvalidJSONError, RawError, ToRawError},
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
mod format;
mod help;
mod move_part;
mod name;
mod part_check;
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

	fn run_wrapper(command: Command, data: Value) -> Result<Response, RawError> {
		match command {
			Command::Version => Ok(VersionRequest::from_json(data)?.run()),
			Command::Print => Ok(PrintRequest::from_json(data)?.run()),
			Command::AlignCheck => Ok(AlignCheckRequest::from_json(data)?.run()),
			Command::Name => Ok(NameRequest::from_json(data)?.run()),
			Command::CreateTable => Ok(CreateTableRequest::from_json(data)?.run()),
			Command::Rescue => Ok(RescueRequest::from_json(data)?.run()),
			Command::ResizePart => Ok(ResizePartRequest::from_json(data)?.run()),
			Command::DeletePart => Ok(DeletePartRequest::from_json(data)?.run()),
			Command::SetFlag => Ok(SetFlagRequest::from_json(data)?.run()),
			Command::SetPartFlag => Ok(SetPartFlagRequest::from_json(data)?.run()),
			Command::ToggleFlag => Ok(ToggleFlagRequest::from_json(data)?.run()),
			Command::TogglePartFlag => Ok(TogglePartFlagRequest::from_json(data)?.run()),
			Command::SetType => Ok(SetTypeRequest::from_json(data)?.run()),
			Command::Help => Ok(HelpRequest::from_json(data)?.run()),
			_ => Err(RawError::new(&data.to_string(), "Unsupported action")),
		}
	}

	pub fn run(data: Value) -> Response {
		let data_command = &data["command"];

		if !data_command.is_string() {
			return Response::new_error(InvalidJSONError::new(&data_command.to_string()));
		}

		let output = Self::run_wrapper(Command::from(data_command.as_str().unwrap()), data);

		if output.is_err() {
			Response::new_error(output.unwrap_err())
		} else {
			output.unwrap()
		}
	}
}

impl ToString for Request {
	fn to_string(&self) -> String {
		self.to_shell_cmd().join(" ")
	}
}

pub trait Runable: Debug + Clone + Into<Request> {
	fn run(&self) -> Response {
		parted_cmd(Into::<Request>::into((*self).clone()).to_shell_cmd())
	}
}
