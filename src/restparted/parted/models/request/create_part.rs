use crate::restparted::{
	model::{
		base::serialize::Deserializable,
		errors::{invalid_json::InvalidJSONError, RawError, ToRawError},
	},
	parted::{
		command::parted_cmd,
		models::{commands::Command, device::Device, response::Response,request::{Request, Runable}},
		system::{device::partition_types::PartitionType, filesystem::FileSystem},
	},
};

#[derive(Clone, Debug)]
pub struct CreatePartRequest {
	pub device: Device,
	pub part_type: Option<PartitionType>,
	pub name: Option<String>,
	pub fs_type: FileSystem,
	pub fs_name: Option<String>,
	pub start: f64,
	pub end: f64,
}

impl From<CreatePartRequest> for Request {
	fn from(item: CreatePartRequest) -> Self {
		let mut args: Vec<String> = Vec::new();

		if item.part_type.is_some() {
			args.push(item.part_type.unwrap().to_string());
		}

		if item.name.is_some() {
			args.push(item.name.unwrap().to_string());
		}

		args.push(item.fs_type.to_string());
		args.push(item.start.to_string());
		args.push(item.end.to_string());

		super::Request {
			command: Command::CreatePart,
			device: item.device,
			arguments: args,
		}
	}
}

impl Deserializable for CreatePartRequest {
	type Error = RawError;

	fn from_json(data: serde_json::Value) -> Result<Self, Self::Error> {
		let data_device = &data["device"];
		let data_part_type = &data["part_type"];
		let data_name = &data["name"];
		let data_fs_type = &data["fs_type"];
		let data_fs_name = &data["fs_name"];
		let data_start = &data["start"];
		let data_end = &data["end"];

		if !data_device.is_string() {
			return Err(InvalidJSONError::new(&data_device.to_string()));
		}

		if !data_start.is_f64() {
			return Err(InvalidJSONError::new(&data_start.to_string()));
		}

		if !data_end.is_f64() {
			return Err(InvalidJSONError::new(&data_end.to_string()));
		}

		let part_type: Option<PartitionType>;
		if data_part_type.is_string() {
			part_type = Some(PartitionType::try_from(data_part_type.as_str().unwrap())?)
		} else {
			part_type = None
		}

		let name: Option<String>;
		if data_name.is_string() {
			name = Some(String::from(data_name.as_str().unwrap()))
		} else {
			name = None
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

		Ok(CreatePartRequest {
			device: Device::try_from(data_device.as_str().unwrap())?,
			part_type,
			name,
			fs_type,
			fs_name,
			start: data_start.as_f64().unwrap(),
			end: data_end.as_f64().unwrap(),
		})
	}
}

impl Runable for CreatePartRequest {
	fn run(&self) -> Response {
		parted_cmd(Request::from(self.clone()).to_shell_cmd());
		todo!("Create part with fs")
	}
}
