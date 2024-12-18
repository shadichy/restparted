use std::{error::Error, process::Command};

use serde_json::Value;

use crate::restparted::{
	model::base::{Deserializable, Serializable},
	parted::models::{
		request::Request,
		response::Response,
	},
};

pub fn run_command(input: Value) -> Result<Value, Box<dyn Error>> {
	Ok(Response::from(
		Command::new("pkexec")
      .arg("parted")
			.arg("--json")
			.args(Request::from_json(input)?.to_shell_cmd())
			.output(),
	)
	.to_json())
}
