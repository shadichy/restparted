use crate::restparted::{model::base::serialize::Serializable, parted::models::commands::Command};
use serde_json::{json, Value};

mod align_check;
mod create_part;
mod create_table;
mod delete_part;
mod format;
mod move_part;
mod name;
mod part_check;
mod print;
mod rescue;
mod resize_part;
mod set_flag;
mod set_part_flag;
mod set_type;
mod toggle_flag;
mod toggle_part_flag;
struct Type(String);

#[allow(non_snake_case)]
impl Type {
	fn BOOL() -> Type {
		Type("Boolean".to_string())
	}
	fn INT() -> Type {
		Type("Integer number".to_string())
	}
	fn UINT() -> Type {
		Type("Non-zero integer".to_string())
	}
	fn FLOAT() -> Type {
		Type("Real number".to_string())
	}
	fn STRING() -> Type {
		Type("String".to_string())
	}

	fn ARRAY(_type: Type) -> Type {
		Type("Array of ".to_string() + &_type.0)
	}

	fn OBJECT(_pairs: Vec<ArgumentDetail>) -> Type {
		Type(
			"Object with ".to_string()
				+ &_pairs
					.iter()
					.map(|d| d.to_pair())
					.collect::<Vec<String>>()
					.join(", "),
		)
	}

	fn ANY(_types: Vec<Type>) -> Type {
		Type(
			"Any of ".to_string()
				+ &_types
					.iter()
					.map(|t| t.to_string())
					.collect::<Vec<String>>()
					.join(", "),
		)
	}
}

impl ToString for Type {
	fn to_string(&self) -> String {
		self.0.clone()
	}
}

struct ArgumentDetail {
	pub key: String,
	optional: bool,
	argument_type: Type,
	detail: String,
}

impl ArgumentDetail {
	fn to_pair(&self) -> String {
		if self.optional {
			self.key.clone() + " [" + &self.argument_type.0 + "]"
		} else {
			self.key.clone() + " " + &self.argument_type.0
		}
	}
}

impl Serializable for ArgumentDetail {
	fn to_json(&self) -> Value {
		json!({
			"optional": self.optional,
			"type": self.argument_type.to_string(),
			"detail": self.detail,
		})
	}
}

struct HelpMessage {
	pub command: Command,
	pub arguments: Vec<ArgumentDetail>,
	pub detail: String,
}

impl Serializable for HelpMessage {
	fn to_json(&self) -> Value {
		let mut data = json!({
			"command": self.command.to_string(),
			"arguments": json!({}),
			"detail": self.detail.clone(),
		});
		for arg in &self.arguments {
			data["arguments"][&arg.key.clone()] = arg.to_json()
		}
		data
	}
}

#[allow(non_snake_case)]
fn HELP() -> HelpMessage {
	HelpMessage {
		command: Command::Help,
		arguments: vec![ArgumentDetail {
			optional: true,
			key: String::from("command"),
			argument_type: Type::STRING(),
			detail: String::from("Show help for command."),
		}],
		detail: String::from("Show help."),
	}
}

#[allow(non_snake_case)]
fn VERSION() -> HelpMessage {
	HelpMessage {
		command: Command::Help,
		arguments: vec![],
		detail: String::from("Show version."),
	}
}

fn require_device(required: bool) -> ArgumentDetail {
	ArgumentDetail {
		optional: !required,
		key: "device".to_string(),
		argument_type: Type::STRING(),
		detail: String::from("Target device to process (disk or partition)."),
	}
}

pub struct HelpCommand;

impl HelpCommand {
	fn query_cmd(command: Command) -> Value {
		match command {
			Command::AlignCheck => align_check::ALIGN_CHECK(),
			Command::Print => print::PRINT(),
			Command::Name => name::NAME(),
			Command::CreateTable => create_table::CREATE_TABLE(),
			Command::CreatePart => create_part::CREATE_PART(),
			Command::Format => format::FORMAT(),
			Command::MovePart => move_part::MOVE_PART(),
			Command::PartCheck => part_check::PART_CHECK(),
			Command::Rescue => rescue::RESCUE(),
			Command::ResizePart => resize_part::RESIZE_PART(),
			Command::DeletePart => delete_part::DELETE_PART(),
			Command::SetFlag => set_flag::SET_FLAG(),
			Command::SetPartFlag => set_part_flag::SET_PART_FLAG(),
			Command::ToggleFlag => toggle_flag::TOGGLE_FLAG(),
			Command::TogglePartFlag => toggle_part_flag::TOGGLE_PART_FLAG(),
			Command::SetType => set_type::SET_TYPE(),
			Command::Help => HELP(),
			Command::Version => VERSION(),
		}
		.to_json()
	}

	pub fn query(command: Option<Command>) -> Value {
		match command {
			None => Value::Null,
			_ => Self::query_cmd(command.unwrap()),
		}
	}
}
