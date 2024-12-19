use std::{error::Error, io, process::Output};

use serde_json::{from_str, json, Value};

use crate::restparted::model::base::serialize::Serializable;

pub struct Response(Value);

impl Response {
	pub fn new(value: Value) -> Response {
		Self(value)
	}

	pub fn new_error<E: Error>(value: E) -> Response {
		Self::from(value.to_string())
	}
}

impl Serializable for Response {
	fn to_json(&self) -> Value {
		self.0.clone()
	}
}

impl From<io::Result<Output>> for Response {
	fn from(item: io::Result<Output>) -> Self {
		if item.is_err() {
			return Response(json!({
			  "message": "Command failed",
			  "error": item.err().unwrap().to_string(),
			}));
		}

		let data = item.unwrap();
		let output = String::from_utf8(data.stdout).unwrap();
		let message: Value;

		if output.starts_with("{") || output.starts_with("[") {
			let message_parse = from_str(
				&(("[".to_owned()
					+ &output
						.replace("\n\n", ",")
						.replace("\n", "")
						.replace(" ", "") + "]")
					.replace(",}", "}")
					.replace(",]", "]")),
			);
			if message_parse.is_ok() {
				message = message_parse.unwrap();
			} else {
				tracing::info!("\n{}", message_parse.unwrap_err());
				message = Value::String(output);
			}
		} else {
			message = Value::String(output);
		}

		Response(json!({
		  "status": data.status.code().unwrap(),
		  "message": message,
		  "warning": String::from_utf8(data.stderr).unwrap(),
		}))
	}
}

impl From<&str> for Response {
	fn from(item: &str) -> Self {
		Response(json!({"message": &item}))
	}
}

impl From<String> for Response {
	fn from(item: String) -> Self {
		Response::from(item.as_str())
	}
}
