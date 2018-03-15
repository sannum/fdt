#![no_std]

extern crate byteorder;
extern crate memchr;

//pub mod error;
pub use node::NodeIterator;
pub use property::PropertyIterator;

mod header;
mod memory_reserve_map;
mod blob;
mod property;
mod node;
mod stringlist;

pub use property::{PropertyValue, IsValue};
use memory_reserve_map::MemoryReserveMap;
use property::Property;

use blob::Blob;
use node::{Node, Subnodes};

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
	/// no other references to this data must exist.
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
	/// ```
	/// use fdt::FDT;
	/// let dtb = include_bytes!("../tests/dt.dtb").as_ptr();
	/// 
	/// unsafe {
	///     let fdt = FDT::from_raw(dtb);
	///     assert!(fdt.is_ok());
	/// }
	/// ```
	/// Using a badly formatted dtb file will cause panic:
	/// ```should_panic
	/// # use fdt::FDT;
	/// let ptr = 0x1234 as *const u8;
	/// unsafe {
	///     let fdt = FDT::from_raw(ptr);
	/// }
	/// ```
	pub unsafe fn from_raw(ptr: *const u8) -> Result<FDT<'buf>, ()> { // FDTError> {
		Ok(FDT { blob: Blob::from_raw(ptr)? })
	}

	/// Returns the physical cpuid of the booting cpu. 
	/// 
	/// If the cpuid isn't available (device tree is of a version < 2) None is returned;
	///
	/// # Examples
	///
	/// ```
	/// use fdt::FDT;
	/// let dtb = include_bytes!("../tests/dt.dtb").as_ptr();
	/// 
	/// let fdt;
	/// unsafe { fdt = FDT::from_raw(dtb).unwrap(); }
	/// 
	/// let boot_cpuid = fdt.boot_cpuid_phys();
	/// ```
	pub fn boot_cpuid_phys(&self) -> Option<u32> {
		self.blob.header().boot_cpuid_phys()
	}
	
	/// Returns the total size in bytes of the flat device tree blob.
	///
	/// # Examples
	///
	/// ```
	/// use fdt::FDT;
	/// let dtb = include_bytes!("../tests/dt.dtb").as_ptr();
	/// 
	/// let fdt;
	/// unsafe { fdt = FDT::from_raw(dtb).unwrap(); }
	/// 
	/// let total_size = fdt.total_size();
	/// // Allocate some memory
	/// ```
	pub fn total_size(&self) -> u32 {
		self.blob.header().totalsize()
	}
	
	/// Returns the reserved memory map of the device tree.
	///
	/// The reserved memory map contains a list of physical memory areas which 
	/// are reserved and should not be allocated for other uses.
	///
	/// The memory reserved map implements Iterator, so all entries can be
	/// accessed using common iterator methods.
	///
	/// # Examples
	///
	/// ```
	/// use fdt::FDT;
	/// let dtb = include_bytes!("../tests/dt.dtb").as_ptr();
	/// 
	/// let fdt;
	/// unsafe { fdt = FDT::from_raw(dtb).unwrap(); }
	/// 
	/// // List all the reserved memory entries in this .dtb (in this case there is only
	/// // one at address 0 with size 4096)
	/// for entry in fdt.memory_reserve_map() {
	///     assert_eq!(entry.address, 0);
	///	    assert_eq!(entry.size, 0x1000);
	/// }
	/// ```
	pub fn memory_reserve_map(&self) -> MemoryReserveMap<'buf>{
		MemoryReserveMap::new(self.blob.rsvmap())
	}
	
	/// Returns a [NodeIterator] over the nodes of the flat device tree.
	///
	/// The nodes are iterated over in a depth first order.
	///
	/// # Examples
	///
	/// ```
	/// use fdt::FDT;
	/// let dtb = include_bytes!("../tests/dt.dtb").as_ptr();
	/// 
	/// let fdt;
	/// unsafe { fdt = FDT::from_raw(dtb).unwrap(); }
	/// 
	/// // Print all nodes
	/// for node in fdt.nodes() {
	///     println!("{}", node.name());
	/// }
	/// ```
	pub fn nodes(&'buf self) -> Subnodes<'buf> {
		Subnodes::new(self.blob.nodes(), 0)
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
	/// ```
	/// use fdt::{FDT, NodeIterator, PropertyValue};
	/// let dtb = include_bytes!("../tests/dt.dtb").as_ptr();
	/// 
	/// let fdt;
	/// unsafe { fdt = FDT::from_raw(dtb).unwrap(); }
	/// 
	/// // Fetch "cpus" node via it's phandle (in this particular .dtb 67)
	/// println!("{}", fdt.phandle(67).unwrap().name()); // prints "cpus"
	/// ```
	pub fn phandle(&'buf self, phandle: u32) -> Option<Node<'buf>> {
		self.nodes().with_phandle(phandle)
	}
		
	/// Takes an alias and returns the corresponding device path
	///
	/// Returns a [None] if the alias doesn't exist in the flat device tree.
	///
	/// # Examples
	///	
	/// ```
	/// use fdt::{FDT, NodeIterator, PropertyValue};
	/// let dtb = include_bytes!("../tests/dt.dtb").as_ptr();
	/// 
	/// let fdt;
	/// unsafe { fdt = FDT::from_raw(dtb).unwrap(); }
	/// 
	/// assert_eq!(fdt.alias("audio"), Some("/soc/audio"));
	///
	/// let audio = fdt.nodes().with_path(fdt.alias("audio").unwrap()).next().unwrap();
	/// assert_eq!(audio.property("compatible").unwrap().parse::<&str>().unwrap(), "brcm,bcm2835-audio\u{0}"); 
	/// ```
	pub fn alias(&'buf self, alias: &str) -> Option<&'buf str> {
		self.nodes().with_path("/aliases").nth(0).and_then(
		|aliases| aliases.property(alias)).and_then(
		|property| property.parse::<&str>().ok()).and_then(
		|string| string.split('\0').nth(0)) // aliases may have trailing null characters
	}
}
