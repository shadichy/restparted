use super::{require_device, HelpMessage};

#[allow(non_snake_case)]
pub fn PRINT() -> HelpMessage {
	HelpMessage {
		command: crate::restparted::parted::models::commands::Command::Print,
		arguments: vec![require_device(false)],
		detail: String::from("Print disk information."),
	}
}
