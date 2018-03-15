use core::str;
use property::{Property, Properties, PropertyIterator, PropertyValue, IsValue};

use blob::{StructReader, Token};

use core::fmt;
use core::cmp::Ordering;

pub struct Node<'buf> {
	name: &'buf str,
	props: StructReader<'buf>,
	subnodes: StructReader<'buf>,
	depth: usize,
}

impl<'buf> Node<'buf> {
	pub fn name(&self) -> &str {
		self.name
	}
	
	/// Checks if the node name matches [name]
	///
	/// If [name] contains an address part ('@' and anything after that), this
	/// function will only match nodes where both name and address parts matches.
	/// Otherwise any nodes with the name [name] will match regardless of address part.
	///
	/// # Examples
	///
	/// todo: match on both specific and non-specific name
	pub fn has_name(&self, name: &str) -> bool {
		let (mut a, mut b) = (self.name.split('@'), name.split('@'));

		match (a.next(), b.next(), a.next(), b.next()) {
			(Some(na), Some(nb), Some(aa), Some(ab)) => na == nb && aa == ab,
			(Some(na), Some(nb), _, None) => na == nb,
			_ => false
		}
	}
	/// Returns an [PropertyIterator] of the properties of the node.
	///
	/// The properties are iterated in an arbitrary order
	///
	/// # Examples
	///
	/// todo: iterate over the first two properties of a node
	pub fn properties(&self) -> Properties<'buf> {
		Properties::new(self.props.clone())
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
	pub fn subnodes(&'buf self) -> Subnodes<'buf> {
		Subnodes::from_node(self)
	}
	
	/// Returns a [NodeIterator] of all supernodes (parents of the node).
	///
	/// The order of iteration is in falling generation, so [.next()] will
	/// return the parent of the current node
	///
	/// # Examples
	///
	/// todo: get the parent of an aliased node
	//fn supernodes(&self) -> impl Iterator<Item=Node>;
	
// utility functions

	/// Returns the property with name [name].
	///
	/// Returns [None] if the node doesn't have the property.
	///
	/// # Examples
	///
	/// todo: get a property from a node
	pub fn property(&self, name: &str) -> Option<Property<'buf>> {
		self.properties().with_name(name).next()
	}
	/// Returns the phandle of the node.
	///
	/// Returns None if the node doesn't have a phandle.
	///
	/// # Examples
	///
	/// todo: get a the phandle from a node and find the node using the phandle
	pub fn phandle(&self) -> Option<u32> {
		self.property("phandle").and_then(|val| val.as_u32().ok())
	}
	
	/// Tests if [compatible] is contained in the nodes [compatible] property.
	///
	/// If the node doesn't have the [combatible] property, false is returned.
	///
	/// # Examples
	///
	/// todo: get a custom property after compatibility has been checked
	pub fn is_compatible_with(&self, name: &str) -> bool {
		self.property("compatible").map_or(false,
			|prop| prop.as_stringlist().contains(name)
		)
	}
	
	/// Returns the #address-cells property value of the node
	///
	/// If the node doen't have the #address-cells property, 2 is assumed as
	/// a default value.
	///
	/// # Examples
	///
	/// todo: get the address_cells property of a node
	pub fn address_cells(&self) -> u32 {
		self.property("#address-cells")
			.and_then(|prop| prop.as_u32().ok())
			.unwrap_or(2)
	}
	
	/// Returns the #size-cells property value of the node
	///
	/// If the node doen't have the #size-cells property, 2 is assumed as
	/// a default value.
	///
	/// # Examples
	///
	/// todo: get the size_cells property of a node
	pub fn size_cells(&self) -> u32 {
		self.property("#size-cells")
			.and_then(|prop| prop.as_u32().ok())
			.unwrap_or(2)
	}
	/// Formats the full path of this node on a [Formatter]
	///
	/// Somewhat expensive since the whole tree up untill this node needs to 
	/// be traversed.
	///
	/// # Examples
	///
	/// todo: print the whole path of a node
	//fn path_format(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result;
	
	/// Returns a [NodeIterator] which iterates the direct descendants of the node.
	///
	/// The iterator skips subnodes which are not direct descendants of the
	/// node. For iteration over all subnodes, use [subnodes()].
	///
	/// # Examples
	///
	/// todo: iterate the children of the root node
	pub fn children(&'buf self) -> Children<'buf> {
		Children(self.subnodes())
	}
	
	/// Returns the supernode with [depth].
	///
	/// Returns None if [depth] is deeper than or equal to the node.
	///
	/// # Examples
	///
	/// todo: return a supernode at depth 2
	pub fn supernode_at_depth(&self, depth: usize) -> Option<Node<'buf>> {
		None
	}
}

impl<'buf> fmt::Display for Node<'buf> {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		writeln!(f, "{:i$}{} {{", "", self.name(), i=self.depth * 2)?;
		for prop in self.properties() {
			writeln!(f, "{:i$}  {}", "", prop, i=self.depth * 2)?;
		}
		writeln!(f, "{:i$}}}", "", i=self.depth * 2)
	}
}

impl<'buf> fmt::Debug for Node<'buf> {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "{}", self.name())
	}
}

#[derive(Clone, Debug)]
pub struct Nodes<'buf> {
	blob: StructReader<'buf>,
	depth: usize,
}

impl<'buf> Nodes<'buf> {
	pub fn new(blob: StructReader<'buf>, initial_depth: usize) -> Nodes<'buf> {
		Nodes {
			blob: blob,
			depth: initial_depth,
		}
	}
	
	fn after_node(node: &'buf Node<'buf>) -> Nodes<'buf> {
		Nodes::new(node.subnodes.clone(), node.depth + 1)
	}
}

impl<'buf> Iterator for Nodes<'buf> {
	type Item = Node<'buf>;
	
	fn next(&mut self) -> Option<Self::Item> {
		loop {
			match self.blob.token() {
				Token::BeginNode => {
					let d = self.depth;
					self.depth += 1;
					return Some(Node {
						name: &self.blob.string(),
						props: self.blob.align(4).clone(),
						subnodes: self.blob.skip_props().clone(),
						depth: d,
					});
				},
				Token::EndNode if self.depth >= 1 => self.depth -= 1,
				Token::End if self.depth == 0 => return None,
				e => panic!("Parse error, unexpected {} at offs {}", e, self.blob.offs()),
			}
		}
	}
}

#[derive(Clone, Debug)]
pub struct Subnodes<'buf> {
	iter: Nodes<'buf>,
	min_depth: usize,
}

impl<'buf, 'path> Subnodes<'buf> {
	fn from_node(node: &'buf Node<'buf>) -> Self {
		Self {
			iter: Nodes::after_node(node),
			min_depth: node.depth + 1,
		}
	}
	
	pub fn new(blob: StructReader<'buf>, depth: usize) -> Self {
		Self {
			iter: Nodes::new(blob, depth),
			min_depth: depth,
		}
	}
	
	pub fn with_path(self, path: &'path str) -> WithPath<'buf, 'path> {
		let depth = self.min_depth;
		WithPath {
			iter: self,
			path: path,
			depth: depth,
		}
	}
}

impl<'buf> Iterator for Subnodes<'buf> {
	type Item = Node<'buf>;

	fn next(&mut self) -> Option<Self::Item> {
		if let Some(n) = self.iter.next() {
			if n.depth >= self.min_depth {
				return Some(n)
			}
		}
		None
	}
}

#[derive(Clone, Debug)]
pub struct Children<'buf> (Subnodes<'buf>);

impl<'buf> Iterator for Children<'buf> {
	type Item = Node<'buf>;
	
	fn next(&mut self) -> Option<Self::Item> {
		let depth = self.0.min_depth + 1;
		self.0.find(|node| node.depth == depth)
	}
}

#[derive(Clone, Debug)]
pub struct WithName<'name, I> {
	iter: I,
	name: &'name str,
}

impl<'name, 'buf, I: Iterator<Item=Node<'buf>>> Iterator for
		WithName<'name, I>
{
	type Item = I::Item;
	
	fn next(&mut self) -> Option<Self::Item> {
		let name = self.name;
		self.iter.find(|node| node.has_name(name))
	}
}

#[derive(Clone, Debug)]
pub struct WithPath<'buf, 'path> {
	iter: Subnodes<'buf>,
	path: &'path str,
	depth: usize,
}

impl<'buf, 'path> Iterator for WithPath<'buf, 'path>
{
	type Item = Node<'buf>;
	
	fn next(&mut self) -> Option<Self::Item> {
		let mut curr = self.path.split('/').nth(self.depth).unwrap();
		let iter = &mut self.iter;
		let depth = &mut self.depth;
		let path = &self.path;
		iter.find(|node| {
			match node.depth().cmp(depth) {
				Ordering::Equal => {
					if node.has_name(curr) || curr == "*" {
						match path.split('/').nth(*depth + 1) {
							Some(name) => {
								*depth += 1;
								curr = name;
								false
							},
							None => true
						}
					} else {
						false
					}/*
					Some(name) => {
						if node.has_name(name) || name == "*" {
							*depth += 1;
							curr = path.split('/').nth(*depth)
						}
						false
					},
					None => true,*/
				},
				Ordering::Less => {
					*depth = node.depth();
					curr = path.split('/').nth(*depth).unwrap();
					false
				},
				Ordering::Greater => false
			}
		})
			
			/*
			if node.depth() == self.depth {
				match curr {
					Some(name) => {
						if node.has_name(name) || name == "*" {
							self.depth += 1;
							curr = self.path.split('/').nth(self.depth)
						}
						return false;
					},
					None => return true,
				}
			} else if node.depth() < self.depth {
				self.depth = node.depth();
				curr = self.path.split('/').nth(self.depth);
				return false;
			}
		})*/
		/*for node in self.iter {
			if node.depth() == self.depth {
				match curr {
					Some(name) => if node.has_name(name) || name == "*" {
						self.depth += 1;
						curr = self.path.split('/').nth(self.depth)
					},
					None => return Some(node)
				}
			} else if node.depth() < self.depth {
				self.depth = node.depth();
				curr = self.path.split('/').nth(self.depth)
			}
		}
		None*/
	}
}

#[derive(Clone, Debug)]
pub struct CompatibleWith<'str, I> {
	iter: I,
	comp: &'str str,
}

impl<'str, 'buf, I: Iterator<Item=Node<'buf>>> Iterator for
		CompatibleWith<'str, I>
{
	type Item = I::Item;
	
	fn next(&mut self) -> Option<Self::Item> {
		let comp = self.comp;
		self.iter.find(|node| node.is_compatible_with(comp))
	}
}

#[derive(Clone, Debug)]
pub struct WithProperty<'name, I> {
	iter: I,
	name: &'name str,
}

impl<'name, 'buf, I: Iterator<Item=Node<'buf>>> Iterator for
		WithProperty<'name, I>
{
	type Item = I::Item;
	
	fn next(&mut self) -> Option<Self::Item> {
		let name = self.name;
		self.iter.find(|node| node.property(name).is_some())
	}
}

impl<'name, I> WithProperty<'name, I> {
	pub fn with_value<V>(self, value: V) -> WithPropertyValue<'name, I, V> {
		WithPropertyValue { iter: self.iter, name: self.name, val: value }
	}
}

#[derive(Clone)]
pub struct WithPropertyValue<'prop, I, V> {
	iter: I,
	name: &'prop str,
	val: V,
}

impl<'buf, I: Iterator<Item=Node<'buf>>, V: IsValue + Copy> Iterator for
		WithPropertyValue<'buf, I, V>
{
	type Item = I::Item;
	
	fn next(&mut self) -> Option<Self::Item> {
		let name = self.name;
		let val = self.val;
		self.iter.find(|node| node.property(name).map_or(false, |prop| prop.is_equal(val)))
	}
}

pub trait NodeIterator<'arg, 'buf>: Iterator<Item=Node<'buf>> {
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
	fn with_name(self, name: &'arg str) -> WithName<'arg, Self> 
		where Self: Sized
	{
		WithName { iter: self, name: name}
	}
	
	/// Filters on nodes compatible with [compatible].
	///
	/// Consumes the iterator and returns an Iterator which iterates over
	/// nodes with a 'compatible' property containing the [compatible] argument.
	///
	/// # Examples
	///
	/// todo: get all nodes compatible with a string.
	fn compatible_with(self, compatible: &'arg str) -> CompatibleWith<'arg, Self> 
			where Self: Sized {
		CompatibleWith { iter: self, comp: compatible }
	}
	
	/// Filters on nodes based on [property].
	///
	/// Consumes the iterator and returns a NodeIterator which iterates over
	/// nodes which contains the property and where that property matches
	/// the value of [property].
	///
	/// # Examples
	///
	/// todo: get some nodes using properties..
	fn with_property(self, property: &'arg str) -> WithProperty<'arg, Self>
			where Self: Sized {
		WithProperty { iter: self, name: property }
	}
	
	/// Returns the node with [phandle] if it is iterated by this iterator.
	///
	/// Consumes the iterator and returns the matching node if it is found,
	/// returns None otherwise.
	///
	/// # Examples
	///
	/// todo: get a node using its phandle.
	fn with_phandle(self, phandle: u32) -> Option<Node<'buf>> where Self: Sized {
		(WithProperty { iter: self, name: "phandle" }).with_value(phandle).nth(0)
	}
}

impl<'name, 'buf, I> NodeIterator<'name, 'buf> for I where I: Iterator<Item=Node<'buf>> {}
