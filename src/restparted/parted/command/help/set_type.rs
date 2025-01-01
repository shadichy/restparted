use super::{require_device, ArgumentDetail, HelpMessage, Type};

#[allow(non_snake_case)]
pub fn SET_TYPE() -> HelpMessage {
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
				key: String::from("type"),
				argument_type: Type::ANY(vec![Type::STRING(), Type::INT()]),
				detail: String::from("The desired type is either hex integer or uuid string."),
			},
    ],
		detail: String::from("Change type (PARTID/PARTUUID) of a partition."),
	}
}
