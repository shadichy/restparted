use crate::restparted::{
	model::{base::serialize::Deserializable, errors::RawError},
	parted::models::{
		commands::Command,
		request::{Request, Runable},
	},
};

#[derive(Clone, Debug)]
pub struct VersionRequest;

impl From<VersionRequest> for Request {
	fn from(_item: VersionRequest) -> Self {
		super::Request::new_cmd(Command::Version)
	}
}

impl Deserializable for VersionRequest {
	type Error = RawError;

	fn from_json(_data: serde_json::Value) -> Result<Self, Self::Error> {
		Ok(VersionRequest {})
	}
}

impl Runable for VersionRequest {}
