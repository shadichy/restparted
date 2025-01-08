pub mod check;
pub mod mkfs;
pub mod move_part;
pub mod resize_part;

pub mod help;

use std::{process::Command, sync::Mutex};

use serde_json::Value;

use crate::restparted::{
	model::base::serialize::Serializable,
	parted::models::{request::Request, response::Response},
	CONFIG,
};

static PARTED_CMD: Mutex<String> = Mutex::new(String::new());

pub fn initialize() {
	*PARTED_CMD.lock().unwrap() = CONFIG.lock().unwrap().parted_executable.clone();
}

pub fn parted_cmd(args: Vec<String>) -> Response {
	Response::from(
		Command::new("pkexec")
			.arg(PARTED_CMD.lock().unwrap().as_str())
			.arg("--json")
			.args(args)
			.output(),
	)
}

pub fn cmd_exists(command: &str) -> bool {
	Command::new("which").arg(command).output().is_ok()
}

pub fn run_query(query: Value) -> Value {
	Request::run(query).to_json()
}
