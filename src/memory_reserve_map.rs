use blob::RsvMapReader;

#[derive(Debug, Clone, Copy)]
pub struct MemoryReserveMapEntry {
	pub address: u64,
	pub size: u64
}

#[derive(Clone)]
pub struct MemoryReserveMap<'blob> {
	blob: RsvMapReader<'blob>,
}

impl<'blob> MemoryReserveMap<'blob> {
	pub fn new(blob: RsvMapReader<'blob>) -> Self {
		Self { blob: blob }
	}
}

impl<'blob> Iterator for MemoryReserveMap<'blob> {
	type Item = MemoryReserveMapEntry;
	
	fn next(&mut self) -> Option<Self::Item> {
		match (self.blob.read_u64(), self.blob.read_u64()) {
			(0, 0) => None,
			(a, s) => Some(Self::Item { address: a, size: s } ),
		}
	}
}
