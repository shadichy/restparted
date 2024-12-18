use std::error::Error;

use crate::restparted::model::base::RawError;

pub mod create_filesystem;

#[allow(non_camel_case_types)]
#[derive(PartialEq, Eq)]
pub enum FileSystem {
	AFFS0 = 0,
	AFFS1 = 1,
	AFFS2 = 2,
	AFFS3 = 3,
	AFFS4 = 4,
	AFFS5 = 5,
	AFFS6 = 6,
	AFFS7 = 7,
	AMUFS = 8,
	AMUFS0 = 9,
	AMUFS1 = 10,
	AMUFS2 = 11,
	AMUFS3 = 12,
	AMUFS4 = 13,
	AMUFS5 = 14,
	APFS1 = 15,
	APFS2 = 16,
	ASFS = 17,
	BTRFS = 18,
	EXT2 = 19,
	EXT3 = 20,
	EXT4 = 21,
	F2FS = 22,
	FAT16 = 23,
	FAT32 = 24,
	HFS = 25,
	HFS_PLUS = 26,
	HFSX = 27,
	HP_UFS = 28,
	JFS = 29,
	LINUX_SWAP = 30,
	LINUX_SWAP_NEW = 31,
	LINUX_SWAP_OLD = 32,
	LINUX_SWAP_V0 = 33,
	LINUX_SWAP_V1 = 34,
	NILFS2 = 35,
	NTFS = 36,
	REISERFS = 37,
	SUN_UFS = 38,
	SWSUSP = 39,
	UDF = 40,
	XFS = 41,
}
impl FileSystem {
	const STR_AFFS0: &'static str = "affs0";
	const STR_AFFS1: &'static str = "affs1";
	const STR_AFFS2: &'static str = "affs2";
	const STR_AFFS3: &'static str = "affs3";
	const STR_AFFS4: &'static str = "affs4";
	const STR_AFFS5: &'static str = "affs5";
	const STR_AFFS6: &'static str = "affs6";
	const STR_AFFS7: &'static str = "affs7";
	const STR_AMUFS0: &'static str = "amufs0";
	const STR_AMUFS1: &'static str = "amufs1";
	const STR_AMUFS2: &'static str = "amufs2";
	const STR_AMUFS3: &'static str = "amufs3";
	const STR_AMUFS4: &'static str = "amufs4";
	const STR_AMUFS5: &'static str = "amufs5";
	const STR_AMUFS: &'static str = "amufs";
	const STR_APFS1: &'static str = "apfs1";
	const STR_APFS2: &'static str = "apfs2";
	const STR_ASFS: &'static str = "asfs";
	const STR_BTRFS: &'static str = "btrfs";
	const STR_EXT2: &'static str = "ext2";
	const STR_EXT3: &'static str = "ext3";
	const STR_EXT4: &'static str = "ext4";
	const STR_F2FS: &'static str = "f2fs";
	const STR_FAT16: &'static str = "fat16";
	const STR_FAT32: &'static str = "fat32";
	const STR_HFS_PLUS: &'static str = "hfs+";
	const STR_HFS: &'static str = "hfs";
	const STR_HFSX: &'static str = "hfsx";
	const STR_HP_UFS: &'static str = "hp-ufs";
	const STR_JFS: &'static str = "jfs";
	const STR_LINUX_SWAP: &'static str = "linux-swap";
	const STR_LINUX_SWAP_NEW: &'static str = "linux-swap(new)";
	const STR_LINUX_SWAP_OLD: &'static str = "linux-swap(old)";
	const STR_LINUX_SWAP_V0: &'static str = "linux-swap(v0)";
	const STR_LINUX_SWAP_V1: &'static str = "linux-swap(v1)";
	const STR_NILFS2: &'static str = "nilfs2";
	const STR_NTFS: &'static str = "ntfs";
	const STR_REISERFS: &'static str = "reiserfs";
	const STR_SUN_UFS: &'static str = "sun-ufs";
	const STR_SWSUSP: &'static str = "swsusp";
	const STR_UDF: &'static str = "udf";
	const STR_XFS: &'static str = "xfs";
}

impl ToString for FileSystem {
	fn to_string(&self) -> String {
		String::from(match self {
			Self::AFFS0 => Self::STR_AFFS0,
			Self::AFFS1 => Self::STR_AFFS1,
			Self::AFFS2 => Self::STR_AFFS2,
			Self::AFFS3 => Self::STR_AFFS3,
			Self::AFFS4 => Self::STR_AFFS4,
			Self::AFFS5 => Self::STR_AFFS5,
			Self::AFFS6 => Self::STR_AFFS6,
			Self::AFFS7 => Self::STR_AFFS7,
			Self::AMUFS0 => Self::STR_AMUFS0,
			Self::AMUFS1 => Self::STR_AMUFS1,
			Self::AMUFS2 => Self::STR_AMUFS2,
			Self::AMUFS3 => Self::STR_AMUFS3,
			Self::AMUFS4 => Self::STR_AMUFS4,
			Self::AMUFS5 => Self::STR_AMUFS5,
			Self::AMUFS => Self::STR_AMUFS,
			Self::APFS1 => Self::STR_APFS1,
			Self::APFS2 => Self::STR_APFS2,
			Self::ASFS => Self::STR_ASFS,
			Self::BTRFS => Self::STR_BTRFS,
			Self::EXT2 => Self::STR_EXT2,
			Self::EXT3 => Self::STR_EXT3,
			Self::EXT4 => Self::STR_EXT4,
			Self::F2FS => Self::STR_F2FS,
			Self::FAT16 => Self::STR_FAT16,
			Self::FAT32 => Self::STR_FAT32,
			Self::HFS_PLUS => Self::STR_HFS_PLUS,
			Self::HFS => Self::STR_HFS,
			Self::HFSX => Self::STR_HFSX,
			Self::HP_UFS => Self::STR_HP_UFS,
			Self::JFS => Self::STR_JFS,
			Self::LINUX_SWAP => Self::STR_LINUX_SWAP,
			Self::LINUX_SWAP_NEW => Self::STR_LINUX_SWAP_NEW,
			Self::LINUX_SWAP_OLD => Self::STR_LINUX_SWAP_OLD,
			Self::LINUX_SWAP_V0 => Self::STR_LINUX_SWAP_V0,
			Self::LINUX_SWAP_V1 => Self::STR_LINUX_SWAP_V1,
			Self::NILFS2 => Self::STR_NILFS2,
			Self::NTFS => Self::STR_NTFS,
			Self::REISERFS => Self::STR_REISERFS,
			Self::SUN_UFS => Self::STR_SUN_UFS,
			Self::SWSUSP => Self::STR_SWSUSP,
			Self::UDF => Self::STR_UDF,
			Self::XFS => Self::STR_XFS,
		})
	}
}

impl TryFrom<&str> for FileSystem {
	type Error = Box<dyn Error>;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		match value {
			Self::STR_AFFS0 => Ok(Self::AFFS0),
			Self::STR_AFFS1 => Ok(Self::AFFS1),
			Self::STR_AFFS2 => Ok(Self::AFFS2),
			Self::STR_AFFS3 => Ok(Self::AFFS3),
			Self::STR_AFFS4 => Ok(Self::AFFS4),
			Self::STR_AFFS5 => Ok(Self::AFFS5),
			Self::STR_AFFS6 => Ok(Self::AFFS6),
			Self::STR_AFFS7 => Ok(Self::AFFS7),
			Self::STR_AMUFS0 => Ok(Self::AMUFS0),
			Self::STR_AMUFS1 => Ok(Self::AMUFS1),
			Self::STR_AMUFS2 => Ok(Self::AMUFS2),
			Self::STR_AMUFS3 => Ok(Self::AMUFS3),
			Self::STR_AMUFS4 => Ok(Self::AMUFS4),
			Self::STR_AMUFS5 => Ok(Self::AMUFS5),
			Self::STR_AMUFS => Ok(Self::AMUFS),
			Self::STR_APFS1 => Ok(Self::APFS1),
			Self::STR_APFS2 => Ok(Self::APFS2),
			Self::STR_ASFS => Ok(Self::ASFS),
			Self::STR_BTRFS => Ok(Self::BTRFS),
			Self::STR_EXT2 => Ok(Self::EXT2),
			Self::STR_EXT3 => Ok(Self::EXT3),
			Self::STR_EXT4 => Ok(Self::EXT4),
			Self::STR_F2FS => Ok(Self::F2FS),
			Self::STR_FAT16 => Ok(Self::FAT16),
			Self::STR_FAT32 => Ok(Self::FAT32),
			Self::STR_HFS_PLUS => Ok(Self::HFS_PLUS),
			Self::STR_HFS => Ok(Self::HFS),
			Self::STR_HFSX => Ok(Self::HFSX),
			Self::STR_HP_UFS => Ok(Self::HP_UFS),
			Self::STR_JFS => Ok(Self::JFS),
			Self::STR_LINUX_SWAP => Ok(Self::LINUX_SWAP),
			Self::STR_LINUX_SWAP_NEW => Ok(Self::LINUX_SWAP_NEW),
			Self::STR_LINUX_SWAP_OLD => Ok(Self::LINUX_SWAP_OLD),
			Self::STR_LINUX_SWAP_V0 => Ok(Self::LINUX_SWAP_V0),
			Self::STR_LINUX_SWAP_V1 => Ok(Self::LINUX_SWAP_V1),
			Self::STR_NILFS2 => Ok(Self::NILFS2),
			Self::STR_NTFS => Ok(Self::NTFS),
			Self::STR_REISERFS => Ok(Self::REISERFS),
			Self::STR_SUN_UFS => Ok(Self::SUN_UFS),
			Self::STR_SWSUSP => Ok(Self::SWSUSP),
			Self::STR_UDF => Ok(Self::UDF),
			Self::STR_XFS => Ok(Self::XFS),
			_ => Err(Box::new(RawError::new(value,"Cannot convert"))),
		}
	}
}

impl TryFrom<String> for FileSystem {
	type Error = Box<dyn Error>;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		Self::try_from(value.as_str())
	}
}
