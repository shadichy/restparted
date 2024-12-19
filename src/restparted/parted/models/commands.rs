#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Command {
	Help = 0,
	Version = 1,
	Print = 2,
	AlignCheck = 3,
	Name = 4,
	CreateTable = 5,
	CreatePart = 6,
	Rescue = 7,
	ResizePart = 8,
	DeletePart = 9,
	SetFlag = 10,
	SetPartFlag = 11,
	ToggleFlag = 12,
	TogglePartFlag = 13,
	SetType = 14,
}

impl Command {
	const STR_VERSION: &'static str = "version";
	const STR_PRINT: &'static str = "print";
	const STR_ALIGN_CHECK: &'static str = "align_check";
	const STR_NAME: &'static str = "name";
	const STR_CREATE_TABLE: &'static str = "create_table";
	const STR_CREATE_PART: &'static str = "create_part";
	const STR_RESCUE: &'static str = "rescue";
	const STR_RESIZE_PART: &'static str = "resize_part";
	const STR_DELETE_PART: &'static str = "delete_part";
	const STR_SET_FLAG: &'static str = "set_flag";
	const STR_SET_PART_FLAG: &'static str = "set_part_flag";
	const STR_TOGGLE_FLAG: &'static str = "toggle_flag";
	const STR_TOGGLE_PART_FLAG: &'static str = "toggle_part_flag";
	const STR_SET_TYPE: &'static str = "set_type";
	const STR_HELP: &'static str = "help";

	pub fn get_real_cmd(&self) -> String {
		String::from(match self {
			Self::Version => "version",
			Self::Print => "print",
			Self::AlignCheck => "align-check",
			Self::Name => "name",
			Self::CreateTable => "mktable",
			Self::CreatePart => "mkpart",
			Self::Rescue => "rescue",
			Self::ResizePart => "resizepart",
			Self::DeletePart => "rm",
			Self::SetFlag => "disk_set",
			Self::SetPartFlag => "set",
			Self::ToggleFlag => "disk_toggle",
			Self::TogglePartFlag => "toggle",
			Self::SetType => "type",
			Self::Help => "help",
		})
	}
}

impl ToString for Command {
	fn to_string(&self) -> String {
		String::from(match self {
			Self::Version => Self::STR_VERSION,
			Self::Print => Self::STR_PRINT,
			Self::AlignCheck => Self::STR_ALIGN_CHECK,
			Self::Name => Self::STR_NAME,
			Self::CreateTable => Self::STR_CREATE_TABLE,
			Self::CreatePart => Self::STR_CREATE_PART,
			Self::Rescue => Self::STR_RESCUE,
			Self::ResizePart => Self::STR_RESIZE_PART,
			Self::DeletePart => Self::STR_DELETE_PART,
			Self::SetFlag => Self::STR_SET_FLAG,
			Self::SetPartFlag => Self::STR_SET_PART_FLAG,
			Self::ToggleFlag => Self::STR_TOGGLE_FLAG,
			Self::TogglePartFlag => Self::STR_TOGGLE_PART_FLAG,
			Self::SetType => Self::STR_SET_TYPE,
			_ => Self::STR_HELP,
		})
	}
}

impl From<&str> for Command {
	fn from(item: &str) -> Self {
		match item {
			Self::STR_VERSION => Self::Version,
			Self::STR_PRINT => Self::Print,
			Self::STR_ALIGN_CHECK => Self::AlignCheck,
			Self::STR_NAME => Self::Name,
			Self::STR_CREATE_TABLE => Self::CreateTable,
			Self::STR_CREATE_PART => Self::CreatePart,
			Self::STR_RESCUE => Self::Rescue,
			Self::STR_RESIZE_PART => Self::ResizePart,
			Self::STR_DELETE_PART => Self::DeletePart,
			Self::STR_SET_FLAG => Self::SetFlag,
			Self::STR_SET_PART_FLAG => Self::SetPartFlag,
			Self::STR_TOGGLE_FLAG => Self::ToggleFlag,
			Self::STR_TOGGLE_PART_FLAG => Self::TogglePartFlag,
			Self::STR_SET_TYPE => Self::SetType,
			_ => Self::Help,
		}
	}
}

impl From<String> for Command {
	fn from(item: String) -> Self {
		Command::from(item.as_str())
	}
}

impl Command {
	pub fn new() -> Self {
		Command::from(String::new())
	}
}
