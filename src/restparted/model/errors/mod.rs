pub mod enum_conversion;
pub mod implements;
pub mod invalid_json;
pub mod invalid_uuid;

use std::{
	error::Error,
	fmt::{self, Display, Formatter},
};

#[derive(Debug, Clone)]
pub struct RawError {
	data: String,
	message: String,
}

impl Display for RawError {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(
			f,
			"{{\"data\":{},\"message\": {}}}",
			self.data, self.message
		)
	}
}

impl Error for RawError {
	fn description(&self) -> &str {
		&self.message
	}
}

impl RawError {
	pub fn new(data: &str, message: &str) -> RawError {
		RawError {
			data: String::from(data),
			message: String::from(message),
		}
	}
}

pub trait ToRawError {
	const MESSAGE: &'static str;
	fn new(value: &str) -> RawError {
		RawError::new(value, Self::MESSAGE)
	}
	fn src(value: &str) -> Box<RawError> {
		Box::new(Self::new(value))
	}
}
