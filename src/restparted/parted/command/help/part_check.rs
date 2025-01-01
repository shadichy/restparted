use super::{require_device, ArgumentDetail, HelpMessage, Type};

#[allow(non_snake_case)]
pub fn PART_CHECK() -> HelpMessage {
	HelpMessage {
    command: crate::restparted::parted::models::commands::Command::PartCheck,
		arguments: vec![
      require_device(true),
			ArgumentDetail {
				optional: false,
				key: String::from("number"),
				argument_type: Type::UINT(),
				detail: String::from("The partition number used by Linux.  On MS-DOS disk labels, the primary partitions number from 1 to 4, logical partitions from 5 onwards."),
			},
    ],
		detail: String::from("Check the status of a partition, and repair if needed."),
	}
}
