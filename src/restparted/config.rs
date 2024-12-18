use serde_json::{from_str, json, Value};
use std::{
	fs::{read_to_string, write},
	path::Path,
};
use tracing::info;

#[derive(Clone)]
pub struct Config {
	// The address to listen on
	pub address: String,

	// The port to listen on
	pub port: u16,

	pub fallback_device: String,

	pub max_worker: usize,
}

impl Config {
	pub const APPNAME: &'static str = "restparted";

	const FILENAME: &'static str = "restparted.jsonc";

	const CONFIG_PATHS: [&'static str; 3] =
		["/etc", "/usr/share/restparted", "/var/lib/restparted"];

	const FALLBACK_DEVICES: [&'static str; 2] = ["/dev/ram0", "/dev/zram0"];

	pub const fn new() -> Self {
		Config {
			address: String::new(),
			port: 0,
			fallback_device: String::new(),
			max_worker: 1,
		}
	}

	fn load_config(&mut self) -> Self {
		for path in Self::CONFIG_PATHS {
			let filepath_str = format!("{}/{}", path, Self::FILENAME);
			info!("Scanning {filepath_str}...");
			let filepath = Path::new(&filepath_str);
			if !filepath.exists() {
				continue;
			}
			let data: Value = from_str(&read_to_string(filepath).unwrap()).unwrap();
			if data["address"].is_string() {
				self.address = data["address"].as_str().unwrap().to_string();
			}
			if data["port"].is_u64() {
				self.port = data["port"].as_u64().unwrap() as u16;
			}
			if data["fallback_device"].is_string() {
				self.fallback_device = data["fallback_device"].as_str().unwrap().to_string();
			}
			break;
		}
		self.clone()
	}

	pub fn from_config_file() -> Self {
		Self::default().load_config()
	}

	pub fn to_json(&self) -> Value {
		json!({
		  "address": self.address,
		  "port":self.port,
		})
	}

	pub fn _write_config(&self) -> Result<(), ()> {
		for path in Self::CONFIG_PATHS {
			let filepath_str = format!("{}/{}", path, Self::FILENAME);
			let filepath = Path::new(&filepath_str);
			if write(filepath, self.to_string()).is_ok() {
				return Ok(());
			}
		}
		Err(())
	}
}

impl Default for Config {
	fn default() -> Self {
		let mut fallback_device = String::new();
		for device in Self::FALLBACK_DEVICES {
			if Path::new(device).exists() {
				fallback_device = device.to_string();
			}
		}
		Config {
			address: String::from("0.0.0.0"),
			port: 8443,
			fallback_device: fallback_device,
			max_worker: 4,
		}
	}
}

impl ToString for Config {
	fn to_string(&self) -> String {
		self.to_json().to_string()
	}
}
