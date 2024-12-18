use std::error::Error;

use crate::restparted::model::base::RawError;

#[allow(non_camel_case_types)]
#[derive(PartialEq, Eq)]
pub enum PartitionFlag {
	BOOT = 0,
	ROOT = 1,
	SWAP = 2,
	HIDDEN = 3,
	RAID = 4,
	LVM = 5,
	LBA = 6,
	HP_SERVICE = 7,
	PALO = 8,
	PREP = 9,
	MSFTRES = 10,
	BIOS_GRUB = 11,
	ATVRECV = 12,
	DIAG = 13,
	LEGACY_BOOT = 14,
	MSFTDATA = 15,
	IRST = 16,
	ESP = 17,
	CHROMEOS_KERNEL = 18,
	BLS_BOOT = 19,
	LINUX_HOME = 20,
	NO_AUTOMOUNT = 21,
}

impl PartitionFlag {
	const STR_BOOT: &'static str = "boot";
	const STR_ROOT: &'static str = "root";
	const STR_SWAP: &'static str = "swap";
	const STR_HIDDEN: &'static str = "hidden";
	const STR_RAID: &'static str = "raid";
	const STR_LVM: &'static str = "lvm";
	const STR_LBA: &'static str = "lba";
	const STR_HP_SERVICE: &'static str = "hp-service";
	const STR_PALO: &'static str = "palo";
	const STR_PREP: &'static str = "prep";
	const STR_MSFTRES: &'static str = "msftres";
	const STR_BIOS_GRUB: &'static str = "bios_grub";
	const STR_ATVRECV: &'static str = "atvrecv";
	const STR_DIAG: &'static str = "diag";
	const STR_LEGACY_BOOT: &'static str = "legacy_boot";
	const STR_MSFTDATA: &'static str = "msftdata";
	const STR_IRST: &'static str = "irst";
	const STR_ESP: &'static str = "esp";
	const STR_CHROMEOS_KERNEL: &'static str = "chromeos_kernel";
	const STR_BLS_BOOT: &'static str = "bls_boot";
	const STR_LINUX_HOME: &'static str = "linux-home";
	const STR_NO_AUTOMOUNT: &'static str = "no_automount";
}

impl ToString for PartitionFlag {
	fn to_string(&self) -> String {
		String::from(match self {
			Self::BOOT => Self::STR_BOOT,
			Self::ROOT => Self::STR_ROOT,
			Self::SWAP => Self::STR_SWAP,
			Self::HIDDEN => Self::STR_HIDDEN,
			Self::RAID => Self::STR_RAID,
			Self::LVM => Self::STR_LVM,
			Self::LBA => Self::STR_LBA,
			Self::HP_SERVICE => Self::STR_HP_SERVICE,
			Self::PALO => Self::STR_PALO,
			Self::PREP => Self::STR_PREP,
			Self::MSFTRES => Self::STR_MSFTRES,
			Self::BIOS_GRUB => Self::STR_BIOS_GRUB,
			Self::ATVRECV => Self::STR_ATVRECV,
			Self::DIAG => Self::STR_DIAG,
			Self::LEGACY_BOOT => Self::STR_LEGACY_BOOT,
			Self::MSFTDATA => Self::STR_MSFTDATA,
			Self::IRST => Self::STR_IRST,
			Self::ESP => Self::STR_ESP,
			Self::CHROMEOS_KERNEL => Self::STR_CHROMEOS_KERNEL,
			Self::BLS_BOOT => Self::STR_BLS_BOOT,
			Self::LINUX_HOME => Self::STR_LINUX_HOME,
			Self::NO_AUTOMOUNT => Self::STR_NO_AUTOMOUNT,
		})
	}
}

impl TryFrom<&str> for PartitionFlag {
	type Error = Box<dyn Error>;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		match value {
			Self::STR_BOOT => Ok(Self::BOOT),
			Self::STR_ROOT => Ok(Self::ROOT),
			Self::STR_SWAP => Ok(Self::SWAP),
			Self::STR_HIDDEN => Ok(Self::HIDDEN),
			Self::STR_RAID => Ok(Self::RAID),
			Self::STR_LVM => Ok(Self::LVM),
			Self::STR_LBA => Ok(Self::LBA),
			Self::STR_HP_SERVICE => Ok(Self::HP_SERVICE),
			Self::STR_PALO => Ok(Self::PALO),
			Self::STR_PREP => Ok(Self::PREP),
			Self::STR_MSFTRES => Ok(Self::MSFTRES),
			Self::STR_BIOS_GRUB => Ok(Self::BIOS_GRUB),
			Self::STR_ATVRECV => Ok(Self::ATVRECV),
			Self::STR_DIAG => Ok(Self::DIAG),
			Self::STR_LEGACY_BOOT => Ok(Self::LEGACY_BOOT),
			Self::STR_MSFTDATA => Ok(Self::MSFTDATA),
			Self::STR_IRST => Ok(Self::IRST),
			Self::STR_ESP => Ok(Self::ESP),
			Self::STR_CHROMEOS_KERNEL => Ok(Self::CHROMEOS_KERNEL),
			Self::STR_BLS_BOOT => Ok(Self::BLS_BOOT),
			Self::STR_LINUX_HOME => Ok(Self::LINUX_HOME),
			Self::STR_NO_AUTOMOUNT => Ok(Self::NO_AUTOMOUNT),
			_ => Err(Box::new(RawError::new(value, "Cannot convert"))),
		}
	}
}

impl TryFrom<String> for PartitionFlag {
	type Error = Box<dyn Error>;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		Self::try_from(value.as_str())
	}
}
