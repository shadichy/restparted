use serde_json::{json, Value};

use crate::restparted::{
	model::{
		base::serialize::{Deserializable, Serializable},
		errors::{
			enum_conversion::EnumConversionError, invalid_json::InvalidJSONError, RawError,
			ToRawError,
		},
	},
	parted::{
		command::parted_cmd,
		models::{commands::Command, device::Device, request::Request, response::Response},
	},
};

use super::Runable;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum AlignCheckType {
	Minimal = 0,
	Optimal = 1,
}

impl AlignCheckType {
	const STR_MINIMAL: &'static str = "minimal";
	const STR_OPTIMAL: &'static str = "optimal";
}

impl ToString for AlignCheckType {
	fn to_string(&self) -> String {
		String::from(match self {
			Self::Minimal => Self::STR_MINIMAL,
			Self::Optimal => Self::STR_OPTIMAL,
		})
	}
}

impl TryFrom<&str> for AlignCheckType {
	type Error = RawError;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		match value {
			Self::STR_MINIMAL => Ok(Self::Minimal),
			Self::STR_OPTIMAL => Ok(Self::Optimal),
			_ => Err(EnumConversionError::new(value)),
		}
	}
}

impl TryFrom<String> for AlignCheckType {
	type Error = RawError;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		Self::try_from(value.as_str())
	}
}

#[derive(Clone, Debug)]
pub struct AlignCheckRequest {
	pub device: Device,
	pub align_check_type: AlignCheckType,
	pub partition_number: u16,
}

impl From<AlignCheckRequest> for Request {
	fn from(item: AlignCheckRequest) -> Self {
		Request {
			command: Command::AlignCheck,
			device: item.device,
			arguments: vec![
				item.align_check_type.to_string(),
				item.partition_number.to_string(),
			],
		}
	}
}

impl Deserializable for AlignCheckRequest {
	type Error = RawError;

	fn from_json(data: serde_json::Value) -> Result<Self, Self::Error> {
		let data_device = &data["device"];
		let data_align_check_type = &data["type"];
		let data_partition_number = &data["number"];

		if !data_device.is_string() {
			return Err(InvalidJSONError::new(&data_device.to_string()));
		}

		if !data_align_check_type.is_string() {
			return Err(InvalidJSONError::new(&data_align_check_type.to_string()));
		}

		if !data_partition_number.is_u64() {
			return Err(InvalidJSONError::new(&data_partition_number.to_string()));
		}

		Ok(AlignCheckRequest {
			device: Device::try_from(data_device.as_str().unwrap())?,
			align_check_type: AlignCheckType::try_from(data_align_check_type.as_str().unwrap())?,
			partition_number: data_partition_number.as_u64().unwrap() as u16,
		})
	}
}

impl Runable for AlignCheckRequest {
	fn run(&self) -> Response {
		let cmd_out = parted_cmd(Into::<Request>::into((*self).clone()).to_shell_cmd()).to_json();
		let out_err = &cmd_out["error"];
		let out_message = &cmd_out["message"];
		let message: Value;
		if out_err.to_string().is_empty() {
			message = Value::Bool(!out_message.as_str().unwrap().contains("not"))
		} else {
			message = out_message.clone()
		}
		Response::new(json!({
		  "message": message,
		  "status": cmd_out["status"],
		  "warning": cmd_out["warning"],
		  "error": out_err,
		}))
	}
}
