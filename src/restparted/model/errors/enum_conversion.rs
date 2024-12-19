use super::ToRawError;

pub struct EnumConversionError;

impl ToRawError for EnumConversionError {
	const MESSAGE: &'static str = "Cannot convert";
}
