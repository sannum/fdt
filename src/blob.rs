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

pub const MIN_COMPAT_VERSION: u32 = 1;
pub const MAX_COMPAT_VERSION: u32 = 17;

const FDT_BEGIN_NODE: u32	= 0x00000001;
const FDT_END_NODE: u32	 	= 0x00000002;
const FDT_PROP: u32			= 0x00000003;
const FDT_NOP: u32			= 0x00000004;
const FDT_END: u32			= 0x00000009;

use core::slice;

use memchr::memchr;
use core::str;
use core::fmt;

use byteorder::{ByteOrder, BE};

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
	
	pub fn nodes(&self) -> StructReader<'buf> {
		let o = self.header().off_dt_struct() as usize;
		let s = self.header().off_dt_strings() as usize;
		StructReader { 
			d: &self.raw[o..],
			s: &self.raw[s..],
			o: 0,
		}
	}
	
	// max_phandle has been omitted for the time beeing, as it's use isn't 
	// considered for the public api (we don't care for overlays).
	// Todo: required for rw operations
	
	/// Retrieve the offset to the memory reserve map
	
	pub fn rsvmap(&self) -> RsvMapReader<'buf> {
		let o = self.header().off_mem_rsvmap() as usize;
		RsvMapReader { d: &self.raw[o..], o: 0 }
	}
}

#[derive(Debug)]
pub enum Token {
	BeginNode,
	EndNode,
	Prop,
	End,
	Error(u32)
}

impl fmt::Display for Token {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		match self {
			&Token::BeginNode => write!(f, "token FDT_BEGIN_NODE"),
			&Token::EndNode => write!(f, "token FDT_END_NODE"),
			&Token::Prop => write!(f, "token FDT_PROP"),
			&Token::End => write!(f, "token FDT_END"),
			&Token::Error(val) => write!(f, "{:x}", val),
		}
	}
}

#[derive(Clone)]
pub struct StructReader<'blob> {
	d: &'blob [u8],
	s: &'blob [u8],
	o: usize,
}

impl<'blob> fmt::Debug for StructReader<'blob> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "StructReader{{ offs: {} }}", self.o)
	}
}

impl<'blob> StructReader<'blob> {
	pub fn offs(&self) -> usize {
		self.o
	}

	pub fn token(&mut self) -> Token {
		loop {
			match BE::read_u32(&self.d[self.o..]) {
				FDT_NOP => {
					self.o += 4;
				},
				FDT_BEGIN_NODE => {
					self.o += 4;
					return Token::BeginNode;
				},
				FDT_END_NODE => {
					self.o += 4;
					return Token::EndNode;
				},
				FDT_PROP => {
					self.o += 4;
					return Token::Prop;
				}
				FDT_END => {
					self.o += 4;
					return Token::End;
				},
				other => {
					self.o += 4;
					return Token::Error(other);
				},
			}
		}
	}
	
	pub fn skip(&mut self, bytes: usize) -> &mut Self {
		self.o += bytes;
		self
	}
	
	pub fn read_u32(&mut self) -> u32 {
		let o = self.o;
		self.o += 4;
		BE::read_u32(&self.d[o..])
	}
	
	pub fn align(&mut self, align: usize) -> &mut Self {
		self.o = (self.o + (align - 1)) & !(align - 1);
		self
	}
	
	pub fn string(&mut self) -> &'blob str {
		let d = &self.d[self.o..];
		let len = memchr(b'\0', d).unwrap_or(d.len());
		self.o += len + 1;
		str::from_utf8(&d[..len]).unwrap()
	}
	
	pub fn skip_props(&mut self) -> &mut Self {
		loop {
			match self.token() {
				Token::Prop => {
					let len = self.read_u32() as usize;
					self.skip(4 + len);
					self.align(4);
				},
				_ => break,
			}
		}
		self.o -= 4;
		self
	}
	
	pub fn slice(&mut self, len: usize) -> &'blob [u8] {
		let d = &self.d[self.o..];
		self.o += len;
		&d[.. len]
	}
	
	pub fn string_ref(&mut self) -> &'blob str {
		let o = self.read_u32() as usize ;
		let d = &self.s[o..];
		let len = memchr(b'\0', d).unwrap_or(d.len());
		str::from_utf8(&d[0..len]).unwrap()
	}
}

#[derive(Copy, Clone)]
pub struct RsvMapReader<'blob> {
	d: &'blob [u8],
	o: usize,
}

impl<'blob> RsvMapReader<'blob> {
	pub fn read_u64(&mut self) -> u64 {
		let o = self.o;
		self.o += 8;
		BE::read_u64(&self.d[o..])
	}
}
