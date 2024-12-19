use super::ToRawError;

pub struct InvalidJSONError;

impl ToRawError for InvalidJSONError {
	const MESSAGE: &'static str = "Property does not match type";
}
