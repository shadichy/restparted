use std::sync::Mutex;

use crate::restparted::parted::{command::cmd_exists, system::filesystem::FSProp};

use super::UNSUPPORTED;

static SUPPORTED: Mutex<FSProp> = Mutex::new(UNSUPPORTED());

pub fn initialize() {
  let can_resize = cmd_exists("ntfsresize");
	*SUPPORTED.lock().unwrap() = FSProp {
		can_create: cmd_exists("mkfs.ntfs"),
		can_check: cmd_exists("ntfsfix"),
		can_grow: can_resize,
		can_live_grow: false,
		can_shrink: can_resize,
		min_size_mb: 1,
		max_size_mb: 268_435_456,
	};
}


pub fn NTFS() -> FSProp {
	SUPPORTED.lock().unwrap().clone()
}
