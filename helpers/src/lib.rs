#![feature(array_from_fn)]

use std::{
	fmt::{Debug, Display},
	io::Read,
};

pub use itertools;
pub use reformation;
pub use regex;

pub type BoxErr = Box<dyn std::error::Error>;

pub fn read_stdin() -> Result<String, BoxErr> {
	let mut buf = String::new();
	std::io::stdin().lock().read_to_string(&mut buf)?;
	Ok(buf)
}

pub fn display<T: Display>(value: T) {
	println!("{}", value);
}

pub fn debug<T: Debug>(value: T) {
	println!("{:?}", value);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayFromError {
	NotEnoughItems,
	TooManyItems,
	OtherError(&'static str),
}

impl std::error::Error for ArrayFromError {}

impl Display for ArrayFromError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(match self {
			ArrayFromError::NotEnoughItems => "Not enough items",
			ArrayFromError::TooManyItems => "Too many items",
			ArrayFromError::OtherError(s) => s,
		})
	}
}

pub trait ArrayFrom<'a> {
	type Item;
	fn array_from<const N: usize>(self) -> Result<[Self::Item; N], ArrayFromError>
	where
		Self: Sized;
}

impl<'a, T: IntoIterator + 'a> ArrayFrom<'a> for T {
	type Item = <Self as IntoIterator>::Item;
	fn array_from<const N: usize>(self) -> Result<[Self::Item; N], ArrayFromError>
	where
		Self: Sized,
	{
		let mut iter = self.into_iter();
		let arr = std::array::try_from_fn(|_| iter.next().ok_or(ArrayFromError::NotEnoughItems))?;

		if iter.next().is_some() {
			return Err(ArrayFromError::TooManyItems);
		}

		Ok(arr)
	}
}
