use byteorder::{ByteOrder, BE};

use blob::Blob;
use blob::align;

use core::mem::size_of;
use core::cmp::Eq;

use core::fmt;
use core::str;

use ::filters::Name;
use ::stringlist::StringList;

pub struct Property<'a> {
	name: &'a str,
	value: &'a [u8],
}

impl<'a> Property<'a> {
	pub fn name(&self) -> &'a str {
		self.name
	}
	
	pub fn raw(&self) -> &'a [u8] {
		self.value
	}
	
	pub fn as_u32(&self) -> Result<u32, &str> {
		if self.value.len() < size_of::<u32>() {
			Err("Parse error, property value to small to be parsed as u32")
		} else {
			Ok(BE::read_u32(self.value))
		}
	}
	
	pub fn as_str(&self) -> &'a str {
		str::from_utf8(self.value).unwrap()
	}
	
	pub fn as_stringlist(&self) -> StringList<'a> {
		StringList::from_utf8(self.value).unwrap()
	}
}

impl<'a> Name for Property<'a> {
	fn name(&self) -> &str {
		self.name
	}
}

impl<'a> fmt::Display for Property<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "{}: ", self.name());
		match self.name() {
			"compatible" 	=> write!(f, "{}", self.as_stringlist()), //todo: stringlist
			"model" 		=> write!(f, "{}", self.as_str()),
			"phandle" 		=> write!(f, "{}", self.as_u32().unwrap()),
			"status" 		=> write!(f, "{}", self.as_str()),
			"#address-cells" => write!(f, "{}", self.as_u32().unwrap()),
			"#size-cells" 	=> write!(f, "{}", self.as_u32().unwrap()),
			"reg" 			=> write!(f, "{:?}", self.value), // todo: prop_enc_array
			"virtual-reg" 	=> write!(f, "{}", self.as_u32().unwrap()),
			"ranges" 		=> write!(f, "[{} bytes]", self.value.len()), // todo: prop_enc_array
			"dma-ranges"	=> write!(f, "[{} bytes]", self.value.len()), // todo: prop_enc_array
			"name"			=> write!(f, "{}", self.as_str()),
			"device_type"	=> write!(f, "{}", self.as_str()),
			"interrupts"	=> write!(f, "{:?}", self.value),
			"interrupt-parent" => write!(f, "{}", self.as_u32().unwrap()),
			"interrupts-extended" => write!(f, "{:?}", self.value),
			"#interrupt-cells" => write!(f, "{}", self.as_u32().unwrap()),
			"interrupt-controller" => write!(f, "{:?}", self.value),
			_ => write!(f, "{:?}", self.value),
		}
	}
}
/*
pub struct PropertyIterator<'buf> {
	blob: Blob<'buf>,
	offs: usize,
}*/

pub struct Properties<'buf> {
	blob: &'buf Blob<'buf>,
	offs: usize,
}

impl<'buf> Properties<'buf> {
	pub fn new(blob: &'buf Blob<'buf>, offs: usize) -> Self {
		Properties {blob: blob, offs: offs}
	}
}

impl<'buf> Iterator for Properties<'buf> {
	type Item = Property<'buf>;
	
	fn next(&mut self) -> Option<Self::Item> {
		let d = self.blob.nodes();
		let mut o = self.offs;

		let val;
		let name_offs;
		
		loop {
			match BE::read_u32(&d[o..]) {
				::FDT_NOP => {
					o += 4;
				},
				::FDT_PROP => {
					o += 4;
					let len = BE::read_u32(&d[o..]) as usize;
					o += 4;
					name_offs = BE::read_u32(&d[o..]) as usize;
					o += 4;
					val = &d[o..o + len];
					o = align(o + len, 4);
					break;
				},
				_ => return None
			}
		}
		if let Ok(name) = self.blob.string(name_offs) {
			let next = Property {
				name : name,
				value : val
			};
			self.offs = o;
			return Some(next)
		} else {
			panic!("Parse error, property name at invalid string offset {}", name_offs);
		}
	}
}

/*
// let a: u32 = node.properties().with_name("a").into()
// node.properties().with_name("a").has_value(a)

// Omitted, covered by iterator
pub fn fdt_next_property_offset(...)

// Omitted, covered by iterator
pub fn fdt_get_property_by_offset(..)

// omitted, covered by fdt::Node::properties().with_name_starting()
pub fn fdt_get_property_namelen(...)

// omitted, covered by fdt::Node::properties().with_name
pub fn fdt_get_property(...)

// omitted, covered by fdt::Property::value()
pub fn fdt_getprop_by_offset(...)

// omitted, covered by fdt::Node::properties().with_name_starting().value
pub fn fdt_getprop_namelen(...)
pub fn fdt_getprop(...)
*/
