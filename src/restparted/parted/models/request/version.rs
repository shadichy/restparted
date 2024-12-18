use std::error::Error;

use crate::restparted::{model::base::Deserializable, parted::models::commands::Command};

use super::Request;

pub struct VersionRequest;

impl From<VersionRequest> for Request {
	fn from(_item: VersionRequest) -> Self {
		super::Request::new_cmd(Command::Version)
	}
}

impl Deserializable for VersionRequest {
	type Error = Box<dyn Error>;

	fn from_json(_data: serde_json::Value) -> Result<Self, Self::Error> {
		Ok(VersionRequest{})
	}
}
