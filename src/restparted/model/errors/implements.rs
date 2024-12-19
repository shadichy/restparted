use std::{
	char::ParseCharError,
	num::{ParseFloatError, ParseIntError},
	str::ParseBoolError,
	string::ParseError,
};

use super::RawError;

impl From<regex::Error> for RawError {
	fn from(value: regex::Error) -> Self {
		RawError::new(&value.to_string(), "Regex parsing failed")
	}
}

impl From<()> for RawError {
	fn from(_item: ()) -> Self {
		RawError::new("", "")
	}
}

impl From<ParseIntError> for RawError {
	fn from(item: ParseIntError) -> Self {
		RawError::new(&item.to_string(), "Failed to parse to integer")
	}
}

impl From<ParseBoolError> for RawError {
	fn from(item: ParseBoolError) -> Self {
		RawError::new(&item.to_string(), "Failed to parse to boolean")
	}
}

impl From<ParseCharError> for RawError {
	fn from(item: ParseCharError) -> Self {
		RawError::new(&item.to_string(), "Failed to parse to character")
	}
}

impl From<ParseFloatError> for RawError {
	fn from(item: ParseFloatError) -> Self {
		RawError::new(
			&item.to_string(),
			"Failed to parse to floating point number",
		)
	}
}

impl From<ParseError> for RawError {
	fn from(item: ParseError) -> Self {
		RawError::new(&item.to_string(), "Failed to parse to string")
	}
}
