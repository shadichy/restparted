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

pub struct UUID(String);

impl ToString for UUID {
	fn to_string(&self) -> String {
		self.0.clone()
	}
}

impl TryFrom<&str> for UUID {
	type Error = Box<dyn Error>;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		if value.len() == 32 {
      let mut str = String::from(&value[..8]);
      str+="-";
      str+=&value[8..12];
      str+="-";
      str+=&value[12..16];
      str+="-";
      str+=&value[16..20];
      str+="-";
      str+=&value[20..];
			Ok(UUID(str))
		} else if value.len() == 36 && Regex::new(r"-")?.find_iter(value).count() == 4 {
			Ok(UUID(String::from(value)))
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
    // format!("{value:x}");
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
		self.0 == other.0
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
