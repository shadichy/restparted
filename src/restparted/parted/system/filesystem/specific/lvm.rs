use std::sync::Mutex;

use crate::restparted::parted::{command::cmd_exists, system::filesystem::FSProp};

use super::UNSUPPORTED;

static SUPPORTED: Mutex<FSProp> = Mutex::new(UNSUPPORTED());

pub fn initialize() {
	if cmd_exists("lvm") {
	*SUPPORTED.lock().unwrap() = FSProp {
		can_create: true,
		can_check: true,
		can_grow: true,
		can_live_grow: true,
		can_shrink: true,
		min_size_mb: 1,
		max_size_mb: 17_592_186_044_416,
	};
	};
}

pub fn LVM2() -> FSProp {
	SUPPORTED.lock().unwrap().clone()
}
