use std::error::Error;

use crate::restparted::model::base::RawError;

#[derive(PartialEq, Eq)]
pub enum PartitionTable {
	AIX = 0,
	AMIGA = 1,
	BSD = 2,
	DVH = 3,
	GPT = 4,
	LOOP = 5,
	MAC = 6,
	MSDOS = 7,
	PC98 = 8,
	SUN = 9,
}

impl PartitionTable {
	const STR_AIX: &'static str = "aix";
	const STR_AMIGA: &'static str = "amiga";
	const STR_BSD: &'static str = "bsd";
	const STR_DVH: &'static str = "dvh";
	const STR_GPT: &'static str = "gpt";
	const STR_LOOP: &'static str = "loop";
	const STR_MAC: &'static str = "mac";
	const STR_MSDOS: &'static str = "msdos";
	const STR_MBR: &'static str = "mbr";
	const STR_PC98: &'static str = "pc98";
	const STR_SUN: &'static str = "sun";
}

impl ToString for PartitionTable {
	fn to_string(&self) -> String {
		String::from(match self {
			Self::AIX => Self::STR_AIX,
			Self::AMIGA => Self::STR_AMIGA,
			Self::BSD => Self::STR_BSD,
			Self::DVH => Self::STR_DVH,
			Self::GPT => Self::STR_GPT,
			Self::LOOP => Self::STR_LOOP,
			Self::MAC => Self::STR_MAC,
			Self::MSDOS => Self::STR_MSDOS,
			Self::PC98 => Self::STR_PC98,
			Self::SUN => Self::STR_SUN,
		})
	}
}

impl TryFrom<&str> for PartitionTable {
	type Error = Box<dyn Error>;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		match value {
			Self::STR_AIX => Ok(Self::AIX),
			Self::STR_AMIGA => Ok(Self::AMIGA),
			Self::STR_BSD => Ok(Self::BSD),
			Self::STR_DVH => Ok(Self::DVH),
			Self::STR_GPT => Ok(Self::GPT),
			Self::STR_LOOP => Ok(Self::LOOP),
			Self::STR_MAC => Ok(Self::MAC),
			Self::STR_MSDOS => Ok(Self::MSDOS),
			Self::STR_MBR => Ok(Self::MSDOS),
			Self::STR_PC98 => Ok(Self::PC98),
			Self::STR_SUN => Ok(Self::SUN),
			_ => Err(Box::new(RawError::new(value, "Cannot convert"))),
		}
	}
}

impl TryFrom<String> for PartitionTable {
	type Error = Box<dyn Error>;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		Self::try_from(value.as_str())
	}
}
