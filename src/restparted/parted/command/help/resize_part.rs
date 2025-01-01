use super::{require_device, ArgumentDetail, HelpMessage, Type};

#[allow(non_snake_case)]
pub fn RESIZE_PART() -> HelpMessage {
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
				key: String::from("end"),
				argument_type: Type::FLOAT(),
				detail: String::from("Location of ending sector of the resized partition, such as 4GB or 10%. Negative values count from the end of the disk."),
			},
    ],
		detail: String::from("Resize a partition."),
	}
}
