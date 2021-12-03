#![feature(array_from_fn)]

use std::{
	fmt::{Debug, Display},
	io::Read,
	str::FromStr,
};

pub use itertools;
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
pub enum MultiParseError {
	NotEnoughItems,
	TooManyItems,
	ParseError,
}

impl std::error::Error for MultiParseError {}

impl Display for MultiParseError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(match self {
			MultiParseError::NotEnoughItems => "Not enough items in multiparse",
			MultiParseError::TooManyItems => "Too many items in multiparse",
			MultiParseError::ParseError => "Parse error in multiparse",
		})
	}
}

pub trait ArrayParse {
	fn array_parse<I: FromStr, const N: usize>(self) -> Result<[I; N], MultiParseError>
	where
		Self: Sized;
}

impl<'a, T: Iterator<Item = &'a str>> ArrayParse for T {
	fn array_parse<I: FromStr, const N: usize>(mut self) -> Result<[I; N], MultiParseError>
	where
		Self: Sized,
	{
		let arr = std::array::try_from_fn(|_| {
			self.next()
				.ok_or(MultiParseError::NotEnoughItems)
				.and_then(|s| s.parse().map_err(|_| MultiParseError::ParseError))
		})?;

		if self.next().is_some() {
			return Err(MultiParseError::TooManyItems);
		}

		Ok(arr)
	}
}
pub trait VecParse {
	fn vec_parse<I: FromStr>(self) -> Result<Vec<I>, MultiParseError>
	where
		Self: Sized;
}

impl<'a, T: Iterator<Item = &'a str>> VecParse for T {
	fn vec_parse<I: FromStr>(self) -> Result<Vec<I>, MultiParseError>
	where
		Self: Sized,
	{
		self.map(FromStr::from_str)
			.collect::<Result<_, _>>()
			.map_err(|_| MultiParseError::ParseError)
	}
}

pub trait TupleFromStr {
	fn tuple_from_str<'a, I: Iterator<Item = &'a str>>(iter: I) -> Result<Self, MultiParseError>
	where
		Self: std::marker::Sized;
}

pub trait TupleParse {
	fn tuple_parse<T>(self) -> Result<T, MultiParseError>
	where
		T: TupleFromStr;
}

impl<'a, I: Iterator<Item = &'a str>> TupleParse for I {
	fn tuple_parse<T>(self) -> Result<T, MultiParseError>
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
	) -> Result<Self, MultiParseError> {
		let tup = ($(
			iter.next()
				.ok_or(MultiParseError::NotEnoughItems)?
				// This turbofish is unnecessary for the generated code, but $i needs to be in here
				// somewhere so the macro knows what to repeat.
				.parse::<$i>()
				.map_err(|_| MultiParseError::ParseError)?,
		)*);
		if iter.next().is_some() {
			return Err(MultiParseError::TooManyItems);
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
// 	) -> Result<Self, MultiParseError> {
// 		let tup = (iter
// 			.next()
// 			.ok_or(MultiParseError::NotEnoughItems)?
// 			.parse()
// 			.map_err(|_| MultiParseError::ParseError)?,);
// 		if iter.next().is_some() {
// 			return Err(MultiParseError::TooManyItems);
// 		}
// 		Ok(tup)
// 	}
// }

// impl<A: FromStr, B: FromStr> TupleFromStr for (A, B) {
// 	fn tuple_from_str<'a, I: Iterator<Item = &'a str>>(
// 		mut iter: I,
// 	) -> Result<Self, MultiParseError> {
// 		let tup = (
// 			iter.next()
// 				.ok_or(MultiParseError::NotEnoughItems)?
// 				.parse()
// 				.map_err(|_| MultiParseError::ParseError)?,
// 			iter.next()
// 				.ok_or(MultiParseError::NotEnoughItems)?
// 				.parse()
// 				.map_err(|_| MultiParseError::ParseError)?,
// 		);
// 		if iter.next().is_some() {
// 			return Err(MultiParseError::TooManyItems);
// 		}
// 		Ok(tup)
// 	}
// }
