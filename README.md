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

libfdt functions covered (this list should be moved/removed when completed):
- [ ] fdt_next_node(..)
- [ ] fdt_fist_subnode(..)
- [ ] fdt_next_subnode(..)
- [ ] fdt_for_each_subnode(..)

- [ ] fdt_get_header(..)
- [ ] fdt_magic(..)
- [ ] fdt_totalsize(..)
- [ ] fdt_off_dt_struct(..)
- [ ] fdt_off_dt_strings(..)
- [ ] fdt_off_mem_rsvmap(..)
- [ ] fdt_version(..)
- [ ] fdt_last_comp_version(..)
- [ ] fdt_boot_cpuid_phys(..)
- [ ] fdt_size_dt_strings(..)
- [ ] fdt_size_dt_struct(..)

- [ ] fdt_check_header(..)
- [ ] fdt_move(..)
- [ ] fdt_string(..)
- [ ] fdt_get_max_phandle(..)
- [ ] fdt_num_mem_rsv(..)
- [ ] fdt_get_mem_rsv(..)
- [ ] fdt_subnode_offset_namelen(..)
- [ ] fdt_subnode_offset(..)
- [ ] fdt_path_offset_namelen(..)
- [ ] fdt_path_offset(..)
- [ ] fdt_get_name(..)
- [ ] fdt_first_property_offset(..)
- [ ] fdt_next_property_offset(..)
- [ ] fdt_for_each_property_offset(..)
- [ ] fdt_get_property_by_offset(..)
- [ ] fdt_get_property_namelen(..)
- [ ] fdt_get_property(..)
- [ ] fdt_getprop_by_offset(..)
- [ ] fdt_getprop_namelen(..)
- [ ] fdt_getprop(..)
- [ ] fdt_get_phandle(..)
- [ ] fdt_get_alias_namelen(..)
- [ ] fdt_get_alias(..)
- [ ] fdt_get_path(..)
- [ ] fdt_supernode_atdepth_offset(..)
- [ ] fdt_node_depth(..)
- [ ] fdt_parent_offset(..)
- [ ] fdt_node_offset_by_prop_value(..)
- [ ] fdt_node_offset_by_phandle(..)
- [ ] fdt_node_check_compatible(..)
- [ ] fdt_node_offset_by_compatible(..)
- [ ] fdt_stringlist_contains(..)
- [ ] fdt_stringlist_count(..)
- [ ] fdt_stringlist_search(..)
- [ ] fdt_stringlist_get(..)
- [ ] fdt_address_cells(..)
- [ ] fdt_size_cells(..)
