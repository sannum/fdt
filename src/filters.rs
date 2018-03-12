use core::marker::PhantomData;
use core::iter::Filter;
use core::fmt;

pub trait Name {
	/// Returns the name of the node
	///
	/// The name includes the address part but excludes the path, if the full
	/// node path is required use [path_format()]
	///
	///
	/// # Examples
	///
	/// todo: get the name of the third and second node
	fn name(&self) -> &str;
}

pub struct WithNameFilter<'name, I, N> {
	iter: I,
	name: &'name str,
	node: PhantomData<N>
}

impl<'name, I: Iterator<Item=N>, N: Name> Iterator for 
		WithNameFilter<'name, I, N> 
{
	type Item = I::Item;
	
	fn next(&mut self) -> Option<N> {
		for x in &mut self.iter {
			if x.name() == self.name {
				return Some(x)
			}
		}
		None
	}
}

impl<'name, I: fmt::Debug, N: Name> fmt::Debug for WithNameFilter<'name, I, N> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("WithNameFilter")
			.field("iter", &self.iter)
			.field("name", &self.name)
			.finish() 
	}
}

pub trait WithName: Iterator {
	fn with_name(self, name: &str) -> WithNameFilter<Self, Self::Item> 
		where Self: Sized,
			Self::Item: Name
	{
		WithNameFilter { iter: self, name: name, node: PhantomData }
	}
}

impl<I> WithName for I where I: Iterator, I::Item: Name {}


pub struct WithPropertyFilter<'prop, I, V: 'prop, N>  {
	iter: I,
	name: &'prop str,
	value: &'prop V,
	node: PhantomData<N>,
}
/*
// Currently, using this iterator will mean that the properties of all nodes
// are parsed twice when iterating through the nodes (once when filtering the
// properties, and once when finding the offset to the next node which, in
// theory, should start right after the last property aligned to 4 bytes.
// But on the other hand, then you couldn't stack filters 
// (i.e node.subnodes().with_name("cpu").with_property(specific_property))
impl<'prop, I: Iterator<Item=N>, V: 'prop, N: Node> Iterator for 
		WithPropertyFilter<'prop, I, V, N> 
{
	type Item = I::Item;
	
	fn next(&mut self) -> Option<N> {
		for node in &mut self.iter {
			if let Some(p) = node.properties()
					.with_name(self.property.name()).next() {
				if p.value() == self.property() {
					return Some(node)
				}
				return None
			}
		}
		None
	}
}

impl<'name, I: fmt::Debug, N: Node> fmt::Debug for WithPropertyFilter<'name, I, N> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("WithPropertyFilter")
			.field("iter", &self.iter)
			.field("property", &self.property)
			.finish() 
	}
}

pub trait WithProperty: Iterator {
	fn with_property(self, property: &Property) -> WithPropertyFilter<Self, Self::Item> 
		where Self: Sized,
			Self::Item: Node
	{
		WithPropertyFilter { iter: self, property: property, node: PhantomData }
	}
}

impl<I> WithProperty for I where I: Iterator {}
*/
