use std::u64;

pub mod affs;
pub mod amufs;
pub mod apfs;
pub mod asfs;
pub mod bcachefs;
pub mod btrfs;
pub mod extfs;
pub mod f2fs;
pub mod fat;
pub mod hfs;
pub mod jfs;
pub mod lvm;
pub mod nilfs2;
pub mod ntfs;
pub mod reiserfs;
pub mod swap;
pub mod swsusp;
pub mod udf;
pub mod ufs;
pub mod xfs;

// pub static CONFIG: Mutex<Config> = Mutex::new(Config::new());
// *CONFIG.lock().unwrap() = Config::from_config_file();
// command::initialize();
// models::device::initialize();

pub fn initialize() {
	apfs::initialize();
	bcachefs::initialize();
	btrfs::initialize();
	extfs::initialize();
	f2fs::initialize();
	fat::initialize();
	hfs::initialize();
	jfs::initialize();
	nilfs2::initialize();
	ntfs::initialize();
	reiserfs::initialize();
	udf::initialize();
	xfs::initialize();
	lvm::initialize();
}

#[derive(Clone)]
pub struct FSProp {
	can_create: bool,
	can_check: bool,
	can_grow: bool,
	can_live_grow: bool,
	can_shrink: bool,
	min_size_mb: u64,
	max_size_mb: u64,
}

impl FSProp {
	pub const fn is_supported(&self) -> bool {
		self.can_create && self.can_check
	}

	pub const fn size_limit_mb(&self) -> (u64, u64) {
		(self.min_size_mb, self.max_size_mb)
	}

	pub const fn can_grow(&self) -> bool {
		self.can_grow
	}

	pub const fn can_live_grow(&self) -> bool {
		self.can_live_grow
	}

	pub const fn can_shrink(&self) -> bool {
		self.can_shrink
	}

	pub fn copy_with(
		&self,
		can_create: Option<bool>,
		can_check: Option<bool>,
		can_grow: Option<bool>,
		can_live_grow: Option<bool>,
		can_shrink: Option<bool>,
		min_size_mb: Option<u64>,
		max_size_mb: Option<u64>,
	) -> Self {
		Self {
			can_create: can_create.unwrap_or(self.can_create),
			can_check: can_check.unwrap_or(self.can_check),
			can_grow: can_grow.unwrap_or(self.can_grow),
			can_live_grow: can_live_grow.unwrap_or(self.can_live_grow),
			can_shrink: can_shrink.unwrap_or(self.can_shrink),
			min_size_mb: min_size_mb.unwrap_or(self.min_size_mb),
			max_size_mb: max_size_mb.unwrap_or(self.max_size_mb),
		}
	}
}

pub const fn CLEARED() -> FSProp {
	FSProp {
		can_create: true,
		can_check: true,
		can_grow: true,
		can_live_grow: false,
		can_shrink: true,
		min_size_mb: 0,
		max_size_mb: u64::MAX,
	}
}

pub const fn UNSUPPORTED() -> FSProp {
	FSProp {
		can_create: false,
		can_check: false,
		can_grow: false,
		can_live_grow: false,
		can_shrink: false,
		min_size_mb: 0,
		max_size_mb: 0,
	}
}

pub const fn FULLY_SUPPORTED() -> FSProp {
	FSProp {
		can_create: true,
		can_check: true,
		can_grow: true,
		can_live_grow: true,
		can_shrink: true,
		min_size_mb: 0,
		max_size_mb: u64::MAX,
	}
}
