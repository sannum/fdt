// returns a pointer to offset within the dt_struct sub-block, returns valid ptr
// if the range offset..offset+checklen is fully contained within the dt_struct block.
// Shouldn't be needed in rust code
pub fn fdt_offset_ptr(fdt: FDT, offset: usize, u32 checklen) -> const* usize {}
	// Todo: Not needed

// returns the next tag and the a pointer to it in nextoffset
// Could be usefull in rust code
pub fn fdt_next_tag(fdt: FDT, offset: usize, nextoffset: const* usize) -> &usize {}


pub struct Blob<'buf> {
	raw: &'buf [u8],
}

impl Blob<'buf> {
	pub fn header(&self) -> Header {
		
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
	pub fn validate(&self) -> Result<(), Error> {
		self.header.magic().valid()?;
		self.header.version().compatible()?;
		Ok(())
	}
		
	// todo: fdt_move intentionally left out until explicitly requested or required

	/// Retrieve a string from the strings block of a device tree
	///
	/// Retrieves the string starting at byte offset 'string_offset'
	/// (native endian) of the strings block or None if 'string_offset'
	/// is out of bounds.
	///
	/// # Excamples
	/// todo:
	pub fn string(&self, string_offset: usize) -> Option<&'buf str> {

	}
	
	// max_phandle has been omitted for the time beeing, as it's use isn't 
	// considered for the public api (we don't care for overlays).
	// Todo: required for rw operations
	
	/// Retrieve the offset to the memory reserve map
	
	pub fn memory_reserve(&self) -> ReserveMap {
	
	}
}
