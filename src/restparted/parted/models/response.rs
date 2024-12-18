use std::{io, process::Output};

use serde_json::{from_str, json, Value};

use crate::restparted::model::base::Serializable;

pub struct Response(Value);

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
			let message_parse = from_str(&output);
			if message_parse.is_ok() {
				message = message_parse.unwrap();
			} else {
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
