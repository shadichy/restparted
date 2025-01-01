use crate::restparted::{
	model::{base::serialize::Deserializable, errors::RawError},
	parted::{command::help::HelpCommand, models::{commands::Command, request::{Request, Runable}, response::Response}},
};

#[derive(Clone, Debug)]
pub struct HelpRequest {
	pub target: Option<Command>,
}

impl From<HelpRequest> for Request {
	fn from(item: HelpRequest) -> Self {
		if item.target == None {
			super::Request::new()
		} else {
			super::Request::new_cmd_arg(Command::Help, vec![item.target.unwrap().to_string()])
		}
	}
}


impl Deserializable for HelpRequest {
	type Error = RawError;

	fn from_json(data: serde_json::Value) -> Result<Self, Self::Error> {
		let data_target = &data["target"];

		let real_target: Option<Command>;
		if data_target.is_string() {
			real_target = Some(Command::from(data_target.as_str().unwrap()));
		} else {
			real_target = None;
		}

		Ok(HelpRequest {
			target: real_target,
		})
	}
}

impl Runable for HelpRequest {
	fn run(&self) -> Response {
		Response::new(HelpCommand::query(self.target.clone()))
	}
}
