use std::{path::Path, sync::Mutex};

use regex::Regex;

use crate::restparted::{model::errors::RawError, CONFIG};

#[derive(PartialEq, Eq, Clone)]
enum DeviceType {
	Default = 0,
	Block = 1,
	Loop = 2,
	ByIdentity = 3,
	MajMin = 4,
}

#[derive(Clone, Debug)]
pub struct Device(String);

static DEFAULT_DEVICE: Mutex<Device> = Mutex::new(Device::new(String::new()));

pub fn initialize() {
	*DEFAULT_DEVICE.lock().unwrap() = Device::new(CONFIG.lock().unwrap().fallback_device.clone());
}

impl Device {
	const DEVFS_PREFIX: &'static str = "/dev/";
	const DEVICE_PREFIX: &'static str = "/dev/block/";
	const SYSFS_DEVICE_PREFIX: &'static str = "/sys/dev/block/";

	pub const fn new(device: String) -> Self {
		Device(device)
	}
}

impl ToString for Device {
	fn to_string(&self) -> String {
		self.0.clone()
	}
}

impl TryFrom<&str> for Device {
	type Error = RawError;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		let dev_type: DeviceType;
		let mut dev_path = String::from(value);
		let starts_with_devfs = value.starts_with(Self::DEVFS_PREFIX);

		if value.contains("loop") {
			dev_type = DeviceType::Loop
		} else if value.contains("by-") {
			dev_type = DeviceType::ByIdentity
		} else if Regex::new(r"[0-9]{1,}:[0-9]{1,}")?.is_match(value) {
			dev_type = DeviceType::MajMin
		} else if starts_with_devfs {
			if value.starts_with(Self::DEVICE_PREFIX) {
				dev_type = DeviceType::Block
			} else {
				dev_type = DeviceType::Default
			}
		} else {
			return Err(RawError::new(value, "Unknown device"));
		}

		if !starts_with_devfs {
			dev_path = String::from(match dev_type {
				DeviceType::MajMin => Self::SYSFS_DEVICE_PREFIX,
				_ => Self::DEVICE_PREFIX,
			}) + value;
		}

		if !Path::new(&dev_path).exists() {
			return Err(RawError::new(value, "Device does not exist"));
		}

		Ok(Device(dev_path))
	}
}

impl TryFrom<String> for Device {
	type Error = RawError;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		Self::try_from(value.as_str())
	}
}

impl Default for Device {
	fn default() -> Self {
		DEFAULT_DEVICE.lock().unwrap().clone()
	}
}
