use std::error::Error;

use crate::restparted::{
	model::base::Deserializable,
	parted::{
		models::{commands::Command, device::Device},
		system::{filesystem::FileSystem, device::partition_types::PartitionType},
	},
};

use super::Request;

pub struct CreatePartRequest {
	pub device: Device,
	pub part_type: Option<PartitionType>,
	pub name: Option<String>,
	pub fs_type: FileSystem,
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
	type Error = Box<dyn Error>;

	fn from_json(data: serde_json::Value) -> Result<Self, Self::Error> {
		let data_device = &data["device"];
		let data_part_type = &data["part_type"];
		let data_name = &data["name"];
		let data_fs_type = &data["fs_type"];
		let data_start = &data["start"];
		let data_end = &data["end"];

		assert!(data_device.is_string() && data_start.is_f64() && data_end.is_f64());

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

		Ok(CreatePartRequest {
			device: Device::try_from(data_device.as_str().unwrap())?,
			part_type,
			name,
			fs_type,
			start: data_start.as_f64().unwrap(),
			end: data_end.as_f64().unwrap(),
		})
	}
}
