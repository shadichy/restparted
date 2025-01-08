use std::sync::Mutex;

use crate::restparted::parted::{command::cmd_exists, system::filesystem::FSProp};

use super::UNSUPPORTED;

static FAT_SUPPORTED: Mutex<FSProp> = Mutex::new(UNSUPPORTED());
static EXFAT_SUPPORTED: Mutex<FSProp> = Mutex::new(UNSUPPORTED());

pub fn initialize() {
	*FAT_SUPPORTED.lock().unwrap() = FSProp {
		can_create: cmd_exists("mkfs.vfat"),
		can_check: cmd_exists("fsck.vfat"),
		can_grow: cmd_exists("fatresize"),
		can_live_grow: false,
		can_shrink: false,
		min_size_mb: 0,
		max_size_mb: 0,
	};
	*EXFAT_SUPPORTED.lock().unwrap() = FSProp {
		can_create: cmd_exists("mkfs.exfat"),
		can_check: cmd_exists("fsck.exfat"),
		can_grow: false,
		can_live_grow: false,
		can_shrink: false,
		min_size_mb: 32,
		max_size_mb: 137_438_953_472,
	};
}

pub fn FAT16() -> FSProp {
	FAT_SUPPORTED.lock().unwrap().copy_with(
		None,
		None,
		None,
		None,
		None,
		Some(8_u64),
		Some(16_384_u64),
	)
}
pub fn FAT32() -> FSProp {
	FAT_SUPPORTED.lock().unwrap().copy_with(
		None,
		None,
		None,
		None,
		None,
		Some(32_u64),
		Some(262_144_u64),
	)
}
pub fn EXFAT() -> FSProp {
	EXFAT_SUPPORTED.lock().unwrap().clone()
}
