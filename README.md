# fdt
Flat device tree parsing in Rust

This library provides an interface for parsing flattened device trees used by many ARM systems and
embedded devices.
It allows for traversing the device tree itself, searching for specific nodes using names, paths
phandles, aliases or properties; parse properties, reading the reserved memory list and 
other common operations. All operations are 'no-std' and without heap allocations, this makes
the library suitable for use in early OS-kernels where memory allocation is not yet set up.

The library strives to achieve functional parity with [libftd](https://github.com/dgibson/dtc) 
used by the linux kernel, albeit at a higher level of abstraction, adhering to Rust principles.
The higher level of abstraction should also come free whenever possible.

At the moment the scope of the library is limited to read only operations.
