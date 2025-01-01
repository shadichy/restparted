use crate::restparted::{
	model::{
		base::serialize::Deserializable,
		errors::{invalid_json::InvalidJSONError, RawError, ToRawError},
	},
	parted::{
		command::parted_cmd,
		models::{commands::Command, device::Device, response::Response,request::{Request, Runable}},
		system::filesystem::FileSystem,
	},
};

#[derive(Clone, Debug)]
pub struct FormatRequest {
	pub device: Device,
	pub fs_type: FileSystem,
	pub fs_name: Option<String>,
}

impl From<FormatRequest> for Request {
	fn from(item: FormatRequest) -> Self {
		super::Request {
			command: Command::CreatePart,
			device: item.device,
			arguments: vec![],
		}
	}
}

impl Deserializable for FormatRequest {
	type Error = RawError;

	fn from_json(data: serde_json::Value) -> Result<Self, Self::Error> {
		let data_device = &data["device"];
		let data_fs_type = &data["type"];
		let data_fs_name = &data["name"];

		if !data_device.is_string() {
			return Err(InvalidJSONError::new(&data_device.to_string()));
		}

		let fs_type: FileSystem;
		if data_fs_type.is_string() {
			fs_type = FileSystem::try_from(data_fs_type.as_str().unwrap())?
		} else {
			fs_type = FileSystem::EXT2
		}

		let fs_name: Option<String>;
		if data_fs_name.is_string() {
			fs_name = Some(String::from(data_fs_name.as_str().unwrap()))
		} else {
			fs_name = None
		}

		Ok(FormatRequest {
			device: Device::try_from(data_device.as_str().unwrap())?,
			fs_type,
			fs_name,
		})
	}
}

impl Runable for FormatRequest {
	fn run(&self) -> Response {
		parted_cmd(Request::from(self.clone()).to_shell_cmd());
		todo!("Format part with mkfs")
	}
}
