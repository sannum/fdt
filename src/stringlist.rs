use core::str;
use core::fmt;

#[derive(Debug)]
pub struct StringList<'a> {
	raw: &'a str,
}

impl<'a> StringList<'a> {
	pub fn from_utf8(data: &'a [u8]) -> Result<Self, str::Utf8Error> {
		str::from_utf8(data).map(|raw| Self { raw: raw })
	}
	
	pub fn strings(&self) -> Strings {
		Strings(self.raw.split_terminator('\0'))
	}

	// todo: scans the strings thrice now, once for string length, once for utf8
	// check and once for string compare, could possibly be done in one pass, 
	// possibly unnecessary optimization
	pub fn contains(&self, key: &str) -> bool {
		self.strings().any(|string| key == string)
	}
}

impl<'a> fmt::Display for StringList<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "[")?;
		let mut strings = self.strings();
		for string in strings.nth(0) {
			write!(f, "\"{}\"", string)?;
		}
		for string in strings {
			write!(f, ", \"{}\"", string)?;
		}
		write!(f, "]")
	}
}

#[derive(Debug)]
pub struct Strings<'a> (str::SplitTerminator<'a, char>);

impl<'a> Iterator for Strings<'a> {
	type Item = &'a str;
	
	fn next(&mut self) -> Option<Self::Item> {
		self.0.next()
	}
}
