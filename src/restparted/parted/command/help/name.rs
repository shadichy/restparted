use super::{require_device, ArgumentDetail, HelpMessage, Type};

#[allow(non_snake_case)]
pub fn NAME() -> HelpMessage {
	HelpMessage {
    command: crate::restparted::parted::models::commands::Command::Name,
		arguments: vec![
      require_device(true),
			ArgumentDetail {
				optional: false,
				key: String::from("number"),
				argument_type: Type::UINT(),
				detail: String::from("The partition number used by Linux.  On MS-DOS disk labels, the primary partitions number from 1 to 4, logical partitions from 5 onwards."),
			},
			ArgumentDetail {
				optional: false,
				key: String::from("label"),
				argument_type: Type::STRING(),
				detail: String::from("The desired name (LABEL) of the partition"),
			},
    ],
		detail: String::from("Rename a partition."),
	}
}
