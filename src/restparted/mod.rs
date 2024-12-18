pub mod argument;
pub mod config;
pub mod model {
	pub mod base;
}
pub mod parted;

use std::sync::Mutex;

use self::config::Config;

pub static CONFIG: Mutex<Config> = Mutex::new(Config::new());

pub fn initialize() {
	*CONFIG.lock().unwrap() = Config::from_config_file();
	parted::models::device::initialize();
}
