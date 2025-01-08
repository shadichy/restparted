use std::sync::Mutex;

use crate::restparted::parted::{command::cmd_exists, system::filesystem::FSProp};

use super::UNSUPPORTED;

static SUPPORTED: Mutex<FSProp> = Mutex::new(UNSUPPORTED());

pub fn initialize() {
	*SUPPORTED.lock().unwrap() = FSProp {
		can_create: cmd_exists("mkfs.f2fs"),
		can_check: cmd_exists("fsck.e2fs"),
		can_grow: cmd_exists("resize.e2fs"),
    can_live_grow: true,
		can_shrink: false,
    min_size_mb:1,
    max_size_mb:17_592_186_044_416,
	};
}

pub fn F2FS() -> FSProp {
	SUPPORTED.lock().unwrap().clone()
}
