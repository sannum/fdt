pub struct Node<'buf> {
	blob: Blob<'buf>,
	name: &'buf str,
	props_offs: usize,
	offs: usize,
	depth: usize,
}

impl<'buf> Node<'buf> {
	/// Returns the name of the node
	///
	/// The name includes the address part but excludes the path, if the full
	/// node path is required use [path_format()]
	///
	///
	/// # Examples
	///
	/// todo: get the name of the third and second node
	pub fn name(&self) -> &'buf str {
		self.name
	}
	
	/// Returns an [PropertyIterator] of the properties of the node.
	///
	/// The properties are iterated in an arbitrary order
	///
	/// # Examples
	///
	/// todo: iterate over the first two properties of a node
	pub fn properties(&self) -> impl Iterator<Item=Property> {
		PropertyIterator::new(self.props_offs)
	}
	
	/// Returns the depth of a node in the tree.
	///
	/// The depth is increasing from 0 at the root node. So a child of the root
	/// node is 1 and a child of that node is 2 etc.
	/// # Examples
	///
	/// todo: assert that root is 0, a child is 1 and a grandchild is 2
	pub fn depth(&self) -> usize {
		self.depth
	}
	
	/// Returns a [NodeIterator] of all subnodes of the node.
	///
	/// All subnodes are iterated in a depth first order.
	///
	/// # Examples
	///
	/// todo: iterate over two children of root
	pub fn subnodes(&self) -> impl NodeIterator {
		let i = SubnodeIterator { 
			blob: self.blob, 
			offs: self.offs, 
			depth: self.depth,
			min_depth: self.depth,
		}
	}
	
	/// Returns a [NodeIterator] of all supernodes (parents of the node).
	///
	/// The order of iteration is in falling generation, so [.next()] will
	/// return the parent of the current node
	///
	/// # Examples
	///
	/// todo: get the parent of an aliased node
	pub fn supernodes(&self) -> impl Iterator<Item=Node> {
	
	}
	
// utility functions

	/// Returns the property with name [name].
	///
	/// Returns [None] if the node doesn't have the property.
	///
	/// # Examples
	///
	/// todo: get a property from a node
	fn property(&self, name: &str) -> Option<Property<'buf>> {}
	
	/// Returns the phandle of the node.
	///
	/// Returns None if the node doesn't have a phandle.
	///
	/// # Examples
	///
	/// todo: get a the phandle from a node and find the node using the phandle
	fn phandle(&self) -> Option<u32> {}
	
	/// Tests if [compatible] is contained in the nodes [compatible] property.
	///
	/// If the node doesn't have the [combatible] property, false is returned.
	///
	/// # Examples
	///
	/// todo: get a custom property after compatibility has been checked
	fn is_compatible(&self, compatible: &str) -> bool {}
	
	/// Returns the #address-cells property value of the node
	///
	/// If the node doen't have the #address-cells property, 2 is assumed as
	/// a default value.
	///
	/// # Examples
	///
	/// todo: get the address_cells property of a node
	fn address_cells(&self) -> u32 {}
	
	/// Returns the #size-cells property value of the node
	///
	/// If the node doen't have the #size-cells property, 2 is assumed as
	/// a default value.
	///
	/// # Examples
	///
	/// todo: get the size_cells property of a node
	fn size_cells(&self) -> u32 {}
	
	/// Formats the full path of this node on a [Formatter]
	///
	/// Somewhat expensive since the whole tree up untill this node needs to 
	/// be traversed.
	///
	/// # Examples
	///
	/// todo: print the whole path of a node
	fn path_format(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {}
	
	/// Returns a [NodeIterator] which iterates the direct descendants of the node.
	///
	/// The iterator skips subnodes which are not direct descendants of the
	/// node. For iteration over all subnodes, use [subnodes()].
	///
	/// # Examples
	///
	/// todo: iterate the children of the root node
	fn children(&self) -> impl Iterator<Item=Node>
	
	/// Returns the supernode with [depth].
	///
	/// Returns None if [depth] is deeper than or equal to the node.
	///
	/// # Examples
	///
	/// todo: return a supernode at depth 2
	fn supernode_at_depth(&self, depth: usize) -> Option<Node {}>
}

pub trait NodeIterator<'buf>: Iterator<Item=Node<'buf> {
	
	/// Filters on nodes with path [path].
	///
	/// Consumes the iterator and returns a NodeIterator which iterates over
	/// nodes with a path matching [path].
	///
	/// If no address part (@xxx) is given in any of the path segments and there
	/// are multiple matching nodes, all matching nodes will be iterated over.
	/// So if a unique node is to be found by path, you should supply the
	/// address part for each path segment.
	///
	/// # Examples
	///
	/// todo: get a node using its path.
	fn with_path(self, path: &str) -> impl NodeIterator<'buf> {
		
	}
	
	/// Filters on nodes with name [name]
	///
	/// Consumes the iterator and returns a NodeIterator which iterates over 
	/// nodes with names matching [path]
	///
	/// An address part (@xxx) can optionally be used to identify a unique node.
	///
	/// # Examples
	///
	/// todo: get some nodes using their names.
	fn with_name(self, name: &str) {}
	
	/// Filters on nodes based on [property].
	///
	/// Consumes the iterator and returns a NodeIterator which iterates over
	/// nodes which contains the property and where that property matches
	/// the value of [property].
	///
	/// # Examples
	///
	/// todo: get some nodes using properties..
	fn with_property(self, property: &'buf Property) {}
	
	/// Returns the node with [phandle] if it is iterated by this iterator.
	///
	/// Consumes the iterator and returns the matching node if it is found,
	/// returns None otherwise.
	///
	/// # Examples
	///
	/// todo: get a node using its phandle.
	fn with_phandle(self, phandle: u32) -> Option<Node<'buf>> {}
	
	/// Filters on nodes compatible with [compatible].
	///
	/// Consumes the iterator and returns a NodeIterator which iterates over
	/// nodes with a 'compatible' property containing the [compatible] arg.
	///
	/// # Examples
	///
	/// todo: get all nodes compatible with a string.
	fn with_compatible(self, compatible: &'buf str) {}
}

struct SubnodeIterator<'buf> {
	blob: Blob<'buf>,
	offs: usize,
	depth: usize,
	min_depth: usize,
}

impl<'buf> SubnodeIterator<'buf> {
	
}

impl<'buf> NodeIterator<'buf> for SubnodeIterator<'buf> {
	fn next(&mut self) -> Option<Node<'buf>> {
		
	}
}

struct SupernodeIterator<'buf> {
	blob: Blob<'buf>,
	offs: usize,
	depth: usize,
}

impl<'buf> SupernodeIterator<'buf> {

}

impl<'buf> NodeIterator<'buf> for SupernodeIterator<'buf> {
	fn next(&mut self) -> Option<Node<'buf>> {
		
	}
}
