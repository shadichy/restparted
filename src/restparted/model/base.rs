use std::{
	error::Error,
	fmt::{self, Debug, Display, Formatter},
};

use regex::Regex;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct RawError {
	data: String,
	message: String,
}

impl Display for RawError {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "{}", self.message)
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

pub trait Serializable {
	fn to_json(&self) -> Value;
	fn serialize(&self) -> Value {
		self.to_json()
	}
}

pub trait Deserializable: Sized {
	type Error;
	fn from_json(data: Value) -> Result<Self, Self::Error>;
	fn deserialize(data: Value) -> Result<Self, Self::Error> {
		Self::from_json(data)
	}
}

pub struct UUID {
	pub value: String,
}

impl ToString for UUID {
	fn to_string(&self) -> String {
		self.value.clone()
	}
}

impl TryFrom<&str> for UUID {
	type Error = Box<dyn Error>;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		if value.len() == 32 {
			// Ok(UUID {
			//   value: String::from(value),
			// });
			todo!()
		} else if value.len() == 35 && Regex::new(r"-")?.find_iter(value).count() == 3 {
			Ok(UUID {
				value: String::from(value),
			})
		} else {
			Err(Box::new(RawError::new(value, "Invalid UUID")))
		}
	}
}

impl TryFrom<String> for UUID {
	type Error = Box<dyn Error>;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		UUID::try_from(value.as_str())
	}
}

impl TryFrom<u128> for UUID {
	type Error = Box<dyn Error>;

	fn try_from(value: u128) -> Result<Self, Self::Error> {
		todo!()
	}
}

impl TryFrom<i128> for UUID {
	type Error = Box<dyn Error>;

	fn try_from(value: i128) -> Result<Self, Self::Error> {
		UUID::try_from(value as u128)
	}
}

impl PartialEq for UUID {
	fn eq(&self, other: &Self) -> bool {
		self.value == other.value
	}
}

impl Eq for UUID {}

// pub fn ternary<T, Fv, Ff>(condition: bool, mut value: Fv, mut fallback: Ff) -> T
// where
// 	Fv: FnMut() -> T,
// 	Ff: FnMut() -> T,
// {
// 	if condition {
// 		value()
// 	} else {
// 		fallback()
// 	}
// }
