use super::{require_device, ArgumentDetail, HelpMessage, Type};

#[allow(non_snake_case)]
pub fn RESCUE() -> HelpMessage {
	HelpMessage {
		command: crate::restparted::parted::models::commands::Command::Rescue,
		arguments: vec![
			require_device(true),
			ArgumentDetail {
				optional: false,
				key: String::from("start"),
				argument_type: Type::FLOAT(),
				detail: String::from("Location of starting sector of the accidentally deleted partition, such as 4GB or 10%. Negative values count from the end of the disk."),
			},
			ArgumentDetail {
				optional: false,
				key: String::from("end"),
				argument_type: Type::FLOAT(),
				detail: String::from("Location of ending sector of the accidentally deleted partition, such as 4GB or 10%. Negative values count from the end of the disk."),
			},
		],
		detail: String::from("Rescue an accidentally deleted partition in location range."),
	}
}
