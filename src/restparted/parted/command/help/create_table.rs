use super::{require_device, ArgumentDetail, HelpMessage, Type};

#[allow(non_snake_case)]
pub fn CREATE_TABLE() -> HelpMessage {
	HelpMessage {
		command: crate::restparted::parted::models::commands::Command::CreateTable,
		arguments: vec![
			require_device(true),
			ArgumentDetail {
				optional: false,
				key: String::from("partition_table"),
				argument_type: Type::STRING(),
				detail: String::from("Desired partition type is one of: aix, amiga, bsd, dvh, gpt, mac, msdos, pc98, sun, atari, loop."),
			},
		],
		detail: String::from("Create partition table on disk."),
	}
}
