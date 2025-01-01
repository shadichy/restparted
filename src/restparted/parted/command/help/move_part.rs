use super::{require_device, ArgumentDetail, HelpMessage, Type};

#[allow(non_snake_case)]
pub fn MOVE_PART() -> HelpMessage {
	HelpMessage {
    command: crate::restparted::parted::models::commands::Command::MovePart,
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
				key: String::from("start"),
				argument_type: Type::FLOAT(),
				detail: String::from("Location of starting sector of the moved partition, such as 4GB or 10%. Negative values count from the end of the disk."),
			},
    ],
		detail: String::from("Move a partition to a new location on the disk."),
	}
}
