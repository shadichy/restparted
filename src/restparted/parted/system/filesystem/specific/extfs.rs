use std::sync::Mutex;

use crate::restparted::parted::{command::cmd_exists, system::filesystem::FSProp};

use super::UNSUPPORTED;

static SUPPORTED: Mutex<FSProp> = Mutex::new(UNSUPPORTED());

pub fn initialize() {
	*SUPPORTED.lock().unwrap() = FSProp {
		can_create: cmd_exists("mke2fs") || cmd_exists("mkfs"),
		can_check: cmd_exists("e2fsck") || cmd_exists("fsck"),
		can_grow: true,
		can_live_grow: false,
		can_shrink: true,
		min_size_mb: 1,
		max_size_mb: 0,
	};
}

pub fn EXT2() -> FSProp {
	SUPPORTED.lock().unwrap().copy_with(
		None,
		None,
		None,
		Some(false),
		None,
		None,
		Some(33_554_432_u64),
	)
}
pub fn EXT3() -> FSProp {
	SUPPORTED.lock().unwrap().copy_with(
		None,
		None,
		None,
		Some(true),
		None,
		None,
		Some(33_554_432_u64),
	)
}
pub fn EXT4() -> FSProp {
	SUPPORTED.lock().unwrap().copy_with(
		None,
		None,
		None,
		Some(true),
		None,
		None,
		Some(72_057_594_037_927_936_u64),
	)
}
