use super::{require_device, ArgumentDetail, HelpMessage, Type};

#[allow(non_snake_case)]
pub fn ALIGN_CHECK() -> HelpMessage {
	HelpMessage {
		command: crate::restparted::parted::models::commands::Command::AlignCheck,
		arguments: vec![
			require_device(true),
			ArgumentDetail {
				optional: false,
				key: String::from("type"),
				argument_type: Type::STRING(),
				detail: String::from("Desired alignment: minimum or optimal."),
			},
      ArgumentDetail {
        optional: true,
        key: String::from("number"),
        argument_type: Type::UINT(),
        detail: String::from("The partition number used by Linux.  On MS-DOS disk labels, the primary partitions number from 1 to 4, logical partitions from 5 onwards. Default to 1."),
      }
		],
		detail: String::from("Check if device is aligned."),
	}
}
