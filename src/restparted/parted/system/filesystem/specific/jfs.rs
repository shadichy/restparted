use std::sync::Mutex;

use crate::restparted::parted::{command::cmd_exists, system::filesystem::FSProp};

use super::UNSUPPORTED;

static SUPPORTED: Mutex<FSProp> = Mutex::new(UNSUPPORTED());

pub fn initialize() {
	*SUPPORTED.lock().unwrap() = FSProp {
		can_create: cmd_exists("mkfs.jfs"),
		can_check: cmd_exists("fsck.jfs"),
		can_grow: cmd_exists("fsadm"),
		can_live_grow: false,
		can_shrink: false,
		min_size_mb: 1,
		max_size_mb: 17_592_186_044_416,
	};
}

pub fn JFS() -> FSProp {
	SUPPORTED.lock().unwrap().clone()
}
