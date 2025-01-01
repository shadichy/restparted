use super::{require_device, ArgumentDetail, HelpMessage, Type};

#[allow(non_snake_case)]
pub fn FORMAT() -> HelpMessage {
	HelpMessage {
    command: crate::restparted::parted::models::commands::Command::Format,
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
				argument_type: Type::STRING(),
				detail: String::from("Filesystem used. Available filesystems: udf, btrfs, bcachefs, nilfs2, ext4, ext3, ext2, f2fs, fat32, fat16, hfsx, hfs+, hfs, jfs, swsusp, linux-swap(v1), linux-swap(v0), ntfs, reiserfs, hp-ufs, sun-ufs, xfs, apfs2, apfs1, asfs, amufs5, amufs4, amufs3, amufs2, amufs1, amufs0, amufs, affs7, affs6, affs5, affs4, affs3, affs2, affs1, affs0, linux-swap, linux-swap(new), linux-swap(old)."),
			},
			ArgumentDetail {
				optional: true,
				key: String::from("name"),
				argument_type: Type::STRING(),
				detail: String::from("Filesystem name (LABEL) of the partition."),
			},
    ],
		detail: String::from("Format partition as filesystem"),
	}
}
