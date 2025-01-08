use std::sync::Mutex;

use crate::restparted::parted::{command::cmd_exists, system::filesystem::FSProp};

use super::UNSUPPORTED;

static SUPPORTED: Mutex<FSProp> = Mutex::new(UNSUPPORTED());

pub fn initialize() {
  let can_resize = cmd_exists("resize_reiserfs");
	*SUPPORTED.lock().unwrap() = FSProp {
		can_create: cmd_exists("mkfs.reiserfs"),
		can_check: cmd_exists("fsck.reiserfs"),
		can_grow: can_resize,
		can_live_grow: false,
		can_shrink: can_resize,
		min_size_mb: 1,
		max_size_mb: 17_592_186_044_416,
	};
}


pub fn REISERFS() -> FSProp {
	SUPPORTED.lock().unwrap().clone()
}
