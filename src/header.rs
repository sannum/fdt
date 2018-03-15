use blob::MIN_COMPAT_VERSION;
use blob::MAX_COMPAT_VERSION;

use byteorder::{ByteOrder, BE};

pub const HEADER_V1_SIZE: usize = 28;
pub const MAGIC: u32 = 0xd00dfeed;

pub struct Header<'blob> {
	raw: &'blob [u8]
}

impl<'blob> Header<'blob> {
	pub fn new(raw: &'blob [u8]) -> Header<'blob> {
		Header {raw: raw}
	}

	pub fn magic(&self) -> Magic {
		Magic { val: BE::read_u32(&self.raw[0..]) }
	}
	
	pub fn totalsize(&self) -> u32 {
		BE::read_u32(&self.raw[4..])
	}
	
	pub fn off_dt_struct(&self) -> u32 {
		BE::read_u32(&self.raw[8..])
	}
	
	pub fn off_dt_strings(&self) -> u32 {
		BE::read_u32(&self.raw[12..])
	}
	
	pub fn off_mem_rsvmap(&self) -> u32 {
		BE::read_u32(&self.raw[16..])
	}
	
	pub fn version(&self) -> Version {
		Version { val: BE::read_u32(&self.raw[20..]) }
	}
	
	pub fn last_comp_version(&self) -> u32 {
		BE::read_u32(&self.raw[24..])
	}
	
	pub fn boot_cpuid_phys(&self) -> Option<u32> {
		if self.version().val >= 2 {
		 	Some(BE::read_u32(&self.raw[28..]))
		} else {
			None
		}
	}
	
	pub fn size_dt_strings(&self) -> Option<u32> {
		if self.version().val >= 3 {
		 	Some(BE::read_u32(&self.raw[32..]))
		} else {
			None
		}
	}
	
	pub fn size_dt_struct(&self) -> Option<u32> {
		if self.version().val >= 17 {
		 	Some(BE::read_u32(&self.raw[28..]))
		} else {
			None
		}
	}
		
	/// Sanity check the device tree or possible device tree
	///
	/// If the buffer passed to the blob passes validation it probably is a
	/// valid flat device tree which can be parsed by this library
	///
	/// # Errors
	/// If the flat device tree magic number missmatch or the version number
	/// is incompatible with this library a matching Error will be returned.
	///
	/// # Examples
	/// todo:
	pub fn validate(self) -> Result<Self, ()> { // Error> {
		self.magic().valid()?;
		self.version().compatible()?;
		Ok(self)
	}
}

pub struct Magic {
	val: u32,
}

impl Magic {
	pub fn valid(&self) -> Result<(), ()> { // Error> {
		match self.val {
			MAGIC => Ok(()),
			other => panic!("Bad magic number in fdt: {}", other), //Error::BadMagic(other),
		}
	}
}

pub struct Version {
	val: u32,
}

impl Version {
	pub fn compatible(&self) -> Result<(), ()> { // Error> {
		match self.val {
			MIN_COMPAT_VERSION ... MAX_COMPAT_VERSION => Ok(()),
			other => panic!("Incompatible fdt version {}", other), //Error::BadVersion(other),
		}
	}
}
