use super::{require_device, ArgumentDetail, HelpMessage, Type};

#[allow(non_snake_case)]
pub fn TOGGLE_FLAG() -> HelpMessage {
	HelpMessage {
    command: crate::restparted::parted::models::commands::Command::MovePart,
		arguments: vec![
      require_device(true),
			ArgumentDetail {
				optional: false,
				key: String::from("flag"),
				argument_type: Type::STRING(),
				detail: String::from("Desired flag is one of: cylinder_alignment, pmbr_boot."),
			},
    ],
		detail: String::from("Change flag state of a flag on a disk device."),
	}
}
