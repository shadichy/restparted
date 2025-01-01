use super::{require_device, ArgumentDetail, HelpMessage, Type};

#[allow(non_snake_case)]
pub fn SET_PART_FLAG() -> HelpMessage {
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
				key: String::from("flag"),
				argument_type: Type::STRING(),
				detail: String::from("Desired flag is one of: boot, root, swap, hidden, raid, lvm, lba, hp-service, palo, prep, msftres, bios_grub, atvrecv, diag, legacy_boot, msftdata, irst, esp, chromeos_kernel, bls_boot, linux-home, no_automount."),
			},
			ArgumentDetail {
				optional: false,
				key: String::from("state"),
				argument_type: Type::BOOL(),
				detail: String::from("Desired state is one of: on, off."),
			},
    ],
		detail: String::from("Change flag of a partition."),
	}
}
