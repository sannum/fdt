use core::error::Error as CoreError;

pub enum Error {
	NotFound, // The requested node or property does not exist : toremove use option
	BadOffset, // The passed node or property is not part of the fdt : toremove use option
	BadPath, // Function was passed a badly formatted path
	BadPhandle, // Function was passed an invalid phandle : toremove use result
	BadState, // Device tree is incomplete : toremove
	Truncated,
	BadMagic(u32), // Magic number missmatch
	BadVersion(u32), // The version is unsupported by the library
	BadStructure, // The structure of the fdt is corrupt, (misnested nodes, or subnodes preceding properties)
	BadLayout, // The fdt has it's sub-blocks in a bad order
	Internal, // Bug in the library causes an internal assertion to fail
	BadNCells, // Device tree has a #address-cells, #size-cells or similar property with a bad format or value
	BadValue, // Device tree has a property with an unexpected value. For example: a property expected to contain a string list is not NUL-terminated within the length of its value.
}
