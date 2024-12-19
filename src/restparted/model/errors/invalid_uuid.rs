use super::ToRawError;


pub struct InvalidUUIDError;

impl ToRawError for InvalidUUIDError {
	const MESSAGE: &'static str = "Invalid UUID";
}
