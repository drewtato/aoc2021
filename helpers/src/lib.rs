#![feature(array_from_fn)]

use std::{
	fmt::{Debug, Display},
	io::Read,
	str::FromStr,
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

pub trait ArrayFrom {
	type Item;
	fn array_from<const N: usize>(self) -> Result<[Self::Item; N], ArrayFromError>
	where
		Self: Sized;
}

impl<T: Iterator> ArrayFrom for T {
	type Item = <Self as Iterator>::Item;
	fn array_from<const N: usize>(mut self) -> Result<[Self::Item; N], ArrayFromError>
	where
		Self: Sized,
	{
		let arr = std::array::try_from_fn(|_| self.next().ok_or(ArrayFromError::NotEnoughItems))?;

		if self.next().is_some() {
			return Err(ArrayFromError::TooManyItems);
		}

		Ok(arr)
	}
}

#[derive(Debug)]
pub enum TupleParseError {
	NotEnoughItems,
	TooManyItems,
	ParseError,
}

impl std::error::Error for TupleParseError {}

impl Display for TupleParseError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(match self {
			TupleParseError::NotEnoughItems => "Not enough items in tuple parse",
			TupleParseError::TooManyItems => "Too many items in tuple parse",
			TupleParseError::ParseError => "Parse error in tuple parse",
		})
	}
}

pub trait TupleFromStr {
	fn tuple_from_str<'a, I: Iterator<Item = &'a str>>(iter: I) -> Result<Self, TupleParseError>
	where
		Self: std::marker::Sized;
}

pub trait TupleParse {
	fn tuple_parse<T>(self) -> Result<T, TupleParseError>
	where
		T: TupleFromStr;
}

impl<'a, I: Iterator<Item = &'a str>> TupleParse for I {
	fn tuple_parse<T>(self) -> Result<T, TupleParseError>
	where
		T: TupleFromStr,
	{
		T::tuple_from_str(self)
	}
}

macro_rules! tuple_from_str_impl {
	($($i:ident,)*) => {
impl<$($i: FromStr,)*> TupleFromStr for ($($i,)*) {
	fn tuple_from_str<'a, It: Iterator<Item = &'a str>>(
		mut iter: It,
	) -> Result<Self, TupleParseError> {
		let tup = ($(
			iter.next()
				.ok_or(TupleParseError::NotEnoughItems)?
				// This is unnecessary for the generated code, but $i needs to be in here somewhere
				// so the macro knows what to repeat.
				.parse::<$i>()
				.map_err(|_| TupleParseError::ParseError)?,
		)*);
		if iter.next().is_some() {
			return Err(TupleParseError::TooManyItems);
		}
		Ok(tup)
	}
}
	};
}

tuple_from_str_impl!(A,);
tuple_from_str_impl!(A, B,);
tuple_from_str_impl!(A, B, C,);
tuple_from_str_impl!(A, B, C, D,);
tuple_from_str_impl!(A, B, C, D, E,);
tuple_from_str_impl!(A, B, C, D, E, F,);
tuple_from_str_impl!(A, B, C, D, E, F, G,);
tuple_from_str_impl!(A, B, C, D, E, F, G, H,);
tuple_from_str_impl!(A, B, C, D, E, F, G, H, I,);
tuple_from_str_impl!(A, B, C, D, E, F, G, H, I, J,);
tuple_from_str_impl!(A, B, C, D, E, F, G, H, I, J, K,);
tuple_from_str_impl!(A, B, C, D, E, F, G, H, I, J, K, L,);

// impl<A: FromStr> TupleFromStr for (A,) {
// 	fn tuple_from_str<'a, I: Iterator<Item = &'a str>>(
// 		mut iter: I,
// 	) -> Result<Self, TupleParseError> {
// 		let tup = (iter
// 			.next()
// 			.ok_or(TupleParseError::NotEnoughItems)?
// 			.parse()
// 			.map_err(|_| TupleParseError::ParseError)?,);
// 		if iter.next().is_some() {
// 			return Err(TupleParseError::TooManyItems);
// 		}
// 		Ok(tup)
// 	}
// }

// impl<A: FromStr, B: FromStr> TupleFromStr for (A, B) {
// 	fn tuple_from_str<'a, I: Iterator<Item = &'a str>>(
// 		mut iter: I,
// 	) -> Result<Self, TupleParseError> {
// 		let tup = (
// 			iter.next()
// 				.ok_or(TupleParseError::NotEnoughItems)?
// 				.parse()
// 				.map_err(|_| TupleParseError::ParseError)?,
// 			iter.next()
// 				.ok_or(TupleParseError::NotEnoughItems)?
// 				.parse()
// 				.map_err(|_| TupleParseError::ParseError)?,
// 		);
// 		if iter.next().is_some() {
// 			return Err(TupleParseError::TooManyItems);
// 		}
// 		Ok(tup)
// 	}
// }
