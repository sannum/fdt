#![no_std]
//#![feature(conservative_impl_trait)]

extern crate byteorder;
extern crate memchr;

const MIN_COMPAT_VERSION: u32 = 1;
const MAX_COMPAT_VERSION: u32 = 17;

const FDT_BEGIN_NODE: u32	= 0x00000001;
const FDT_END_NODE: u32	 	= 0x00000002;
const FDT_PROP: u32			= 0x00000003;
const FDT_NOP: u32			= 0x00000004;
const FDT_END: u32			= 0x00000009;

//pub mod error;
pub mod filters;
//pub use filters::*;
pub use node::Filters;

mod header;
mod blob;
pub mod property;
mod node;
mod stringlist;

use blob::Blob;
use node::{Nodes};

/// An interface for parsing flat device trees from an in memory buffer.
///
/// For the time being the interface is read only and '[no_std]' with no heap
/// allocations so it is usefull in early kernels where no memory allocation is 
/// brought up yet.
pub struct FDT<'buf> {
	blob: Blob<'buf>
}

impl<'buf> FDT<'buf> {
	/// Creates a new FDT from a raw pointer.
	///
	/// This method will also sanity check the data pointed to by the raw pointer
	/// to determine whether the data is a device tree and of a compatible version.
	///
	/// # Safety
	///
	/// This method is unsafe since we will dereference a raw pointer and act
	/// on the data pointed to. The data must be a valid flat device tree and
	/// no other mutable references to this data should exist.
	/// 
	/// # Errors
	///
	/// If the sanity check fails (the pointer isn't pointing on a valid fdt)
	/// this method will return an error. Likewise, if the fdt is of an
	/// incompatible version this method will return an error. Other corruptions
	/// to the binary data is undefined behaivour and are prohibited to ensure
	/// safety.
	///
	/// # Examples
	///
	/// todo: instantiate a FDT from the second argument in a kernel_start?
	pub unsafe fn from_raw(ptr: *const u8) -> Result<FDT<'buf>, ()> { // FDTError> {
		Ok(FDT { blob: Blob::from_raw(ptr)? })
	}

	/// Returns the version of the flat device tree.
	///
	/// # Examples
	///
	/// todo: 
	//fn version(&self) -> u32 {
	//	self.blob.header().version().into_u32()
	//}
	
	/// Returns the physical cpuid of the booting cpu. 
	/// 
	/// If the flat device tree version < 2 this method returns None.
	pub fn boot_cpuid_phys(&self) -> Option<u32> {
		self.blob.header().boot_cpuid_phys()
	}
	
	/// Returns the total size in bytes of the flat device tree blob.
	pub fn total_size(&self) -> u32 {
		self.blob.header().totalsize()
	}
	
	/// Returns the reserved memory map of the device tree.
	///
	/// The reserved memory map contains a list of physical memory areas which 
	/// are reserved and should not be allocated for other uses
	//fn memory_reserve_map(&self) -> MemoryReserveMap<'buf>{
	//	self.blob.memory_reserve_map()
	//}
	
	/// Returns a [NodeIterator] of the nodes of the flat device tree.
	///
	/// The nodes are iterated over in a depth first order.
	pub fn nodes(&'buf self) -> Nodes<'buf> {
		Nodes::from_blob(&self.blob)
	}
	
// Utility methods
	/// Takes a phandle and returns the corresponding device [Node]
	///
	/// Returns a [None] if no device [Node] with the requested phandle exists.
	/// All phandles are assumed to be unique and if multiple nodes share a
	/// phandle value, all but the first one will be ignored.
	///
	/// # Examples
	///
	/// todo: Find a interrupt parent based on phandle
	//fn node_phandle(&self, phandle: u32) -> Option<Node<'buf>> {}
	
	/// Takes an alias and returns the corresponding device [Node]
	///
	/// Returns a [None] if the alias doesn't exist in the flat device tree.
	///
	/// # Examples
	///
	/// todo: Find an aliased node
	//fn node_with_alias(&self, alias: &str) -> Option<Node<'buf>>{}
	
	/// Takes an alias and returns the corresponding device path
	///
	/// Returns a [None] if the alias doesn't exist in the flat device tree.
	///
	/// # Examples
	///
	/// todo: Find an aliased node path
	fn path_from_alias(&self, alias: &str) -> Option<&'buf str> {
		None
	}
}

#[cfg(tests)]
mod tests {
	#[test]
	fn test0() {
		println!("test");
		assert_eq!(2 + 2, 4);
	}
}
