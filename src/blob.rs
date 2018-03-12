// returns a pointer to offset within the dt_struct sub-block, returns valid ptr
// if the range offset..offset+checklen is fully contained within the dt_struct block.
// Shouldn't be needed in rust code
//pub fn fdt_offset_ptr(fdt: FDT, offset: usize, u32 checklen) -> const* usize {}
	// Todo: Not needed

// returns the next tag and the a pointer to it in nextoffset
// Could be usefull in rust code
//pub fn fdt_next_tag(fdt: FDT, offset: usize, nextoffset: const* usize) -> &usize {}

use header::Header;
use header::HEADER_V1_SIZE;

use core::slice;

use memchr::memchr;
use core::str;

#[derive(Debug)]
pub struct Blob<'buf> {
	raw: &'buf [u8],
}

impl<'buf> Blob<'buf> {
	pub unsafe fn from_raw(ptr: *const u8) -> Result<Self, ()> { // FDTError> {
		let h = slice::from_raw_parts(ptr, HEADER_V1_SIZE as usize);
		let th = Header::new(h).validate()?;
		let s = slice::from_raw_parts(ptr, th.totalsize() as usize);
		Ok(Blob {raw: s})
	}

	pub fn header(&self) -> Header {
		Header::new(&self.raw[0..])
	}
		
	// todo: fdt_move intentionally left out until explicitly requested or required

	/// Retrieve a string from the strings block of a device tree
	///
	/// Retrieves the string starting at byte offset 'string_offset'
	/// (native endian) of the strings block or Err(()) if 'string_offset'
	/// is out of bounds.
	///
	/// # Excamples
	/// todo:
	pub fn string(&self, string_offset: usize) -> Result<&'buf str, ()> {
		let o = string_offset + self.header().off_dt_strings() as usize;
		let len = memchr(b'\0', &self.raw[o..]).unwrap();
		Ok(str::from_utf8(&self.raw[o..o + len]).unwrap())
	}
	
	pub fn nodes(&self) -> &'buf [u8] {
		let o = self.header().off_dt_struct() as usize;
		&self.raw[o..]
	}
	
	// max_phandle has been omitted for the time beeing, as it's use isn't 
	// considered for the public api (we don't care for overlays).
	// Todo: required for rw operations
	
	/// Retrieve the offset to the memory reserve map
	
	pub fn rsvmap(&self) -> &'buf [u8] {
		&self.raw[0..]
	}
}

pub fn align(offset: usize, align: usize) -> usize {
	(offset + (align - 1)) & !(align - 1)
}
