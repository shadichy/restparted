use super::{require_device, ArgumentDetail, HelpMessage, Type};

#[allow(non_snake_case)]
pub fn CREATE_PART() -> HelpMessage {
	HelpMessage {
		command: crate::restparted::parted::models::commands::Command::CreatePart,
		arguments: vec![
			require_device(true),
			ArgumentDetail {
				optional: true,
				key: String::from("part_type"),
				argument_type: Type::STRING(),
				detail: String::from("Desired partition type: primary, logical, extended."),
			},
			ArgumentDetail {
				optional: true,
				key: String::from("name"),
				argument_type: Type::STRING(),
				detail: String::from("Name (PARTLABEL) of the partition."),
			},
			ArgumentDetail {
				optional: false,
				key: String::from("fs_type"),
				argument_type: Type::STRING(),
				detail: String::from("Filesystem used. Available filesystems: udf, btrfs, bcachefs, nilfs2, ext4, ext3, ext2, f2fs, fat32, fat16, hfsx, hfs+, hfs, jfs, swsusp, linux-swap(v1), linux-swap(v0), ntfs, reiserfs, hp-ufs, sun-ufs, xfs, apfs2, apfs1, asfs, amufs5, amufs4, amufs3, amufs2, amufs1, amufs0, amufs, affs7, affs6, affs5, affs4, affs3, affs2, affs1, affs0, linux-swap, linux-swap(new), linux-swap(old)."),
			},
			ArgumentDetail {
				optional: true,
				key: String::from("fs_name"),
				argument_type: Type::STRING(),
				detail: String::from("Filesystem name (LABEL) of the partition."),
			},
			ArgumentDetail {
				optional: false,
				key: String::from("start"),
				argument_type: Type::FLOAT(),
				detail: String::from("Location of starting sector of the newly creating partition, such as 4GB or 10%. Negative values count from the end of the disk."),
			},
			ArgumentDetail {
				optional: false,
				key: String::from("end"),
				argument_type: Type::FLOAT(),
				detail: String::from("Location of ending sector of the newly creating partition, such as 4GB or 10%. Negative values count from the end of the disk."),
			},
		],
		detail: String::from("Create a new partition."),
	}
}
