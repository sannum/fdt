extern crate fdt;

use fdt::{FDT, NodeIterator};

const DTB: &'static [u8] = include_bytes!("dt.dtb");

fn init() -> FDT<'static> {
	unsafe {
		FDT::from_raw(DTB.as_ptr()).unwrap()
	}
}

#[test]
fn test_construct() {
	unsafe {
		let fdt = FDT::from_raw(DTB.as_ptr());
		assert!(fdt.is_ok());
	}
}

#[test]
fn test_boot_cpu() {
	let fdt = init();
	assert_eq!(fdt.boot_cpuid_phys().unwrap(), 0);
}

#[test]
fn test_total_size() {
	let fdt = init();
	assert_eq!(fdt.total_size(), 16917);
}

#[test]
fn test_fetching_node() {
	let fdt = init();
	let mut with_name = fdt.nodes().with_name("cpus");
	let cpus = with_name.next();
	assert!(cpus.is_some());
	assert_eq!(cpus.unwrap().name(), "cpus");
	assert!(with_name.next().is_none());
}
