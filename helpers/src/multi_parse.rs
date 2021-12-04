use std::array;

use std::str::FromStr;

use std::fmt::Display;

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

pub trait MultiParse {
	fn multi_parse<T>(self) -> Result<T, MultiParseError>
	where
		T: MultiFromStr;
}

impl<I: IntoIterator<Item = S>, S: AsRef<str>> MultiParse for I {
	/// Returns a collection of parsed values built from the iterator.
	///
	/// If the iterator did not fit exactly into the collection, or if the parse failed, this
	/// returns an error. For arbitrary-length collections such as [Vec], only parse errors are
	/// encountered.
	fn multi_parse<T>(self) -> Result<T, MultiParseError>
	where
		T: MultiFromStr,
	{
		MultiFromStr::multi_from_str(self.into_iter())
	}
}

pub trait MultiFromStr {
	fn multi_from_str<I, S>(iter: I) -> Result<Self, MultiParseError>
	where
		Self: Sized,
		I: IntoIterator<Item = S>,
		S: AsRef<str>;
}

impl<T: FromStr> MultiFromStr for Vec<T> {
	fn multi_from_str<I, S>(iter: I) -> Result<Self, MultiParseError>
	where
		Self: Sized,
		I: IntoIterator<Item = S>,
		S: AsRef<str>,
	{
		iter.into_iter()
			.map(|s| s.as_ref().parse().map_err(|_| MultiParseError::ParseError))
			.collect()
	}
}

impl<T: FromStr, const N: usize> MultiFromStr for [T; N] {
	fn multi_from_str<I, S>(iter: I) -> Result<Self, MultiParseError>
	where
		Self: Sized,
		I: IntoIterator<Item = S>,
		S: AsRef<str>,
	{
		let mut iter = iter.into_iter();
		array::try_from_fn(|_| {
			iter.next()
				.ok_or(MultiParseError::NotEnoughItems)
				.and_then(|s| s.as_ref().parse().map_err(|_| MultiParseError::ParseError))
		})
	}
}

macro_rules! tuple_multi_from_str_impl {
	    ($($i:ident,)*) => {
		    impl<$($i: FromStr,)*> MultiFromStr for ($($i,)*) {
			    fn multi_from_str<I, S>(iter: I) -> Result<Self, MultiParseError>
			    where
				    Self: Sized,
				    I: IntoIterator<Item = S>,
				    S: AsRef<str>,
			    {
				    let mut iter = iter.into_iter();
				    let tup = ($(
					    iter.next()
						    .ok_or(MultiParseError::NotEnoughItems)?
						    .as_ref()
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

tuple_multi_from_str_impl!();
tuple_multi_from_str_impl!(A0,);
tuple_multi_from_str_impl!(A0, A1,);
tuple_multi_from_str_impl!(A0, A1, A2,);
tuple_multi_from_str_impl!(A0, A1, A2, A3,);
tuple_multi_from_str_impl!(A0, A1, A2, A3, A4,);
tuple_multi_from_str_impl!(A0, A1, A2, A3, A4, A5,);
tuple_multi_from_str_impl!(A0, A1, A2, A3, A4, A5, A6,);
tuple_multi_from_str_impl!(A0, A1, A2, A3, A4, A5, A6, A7,);
tuple_multi_from_str_impl!(A0, A1, A2, A3, A4, A5, A6, A7, A8,);
tuple_multi_from_str_impl!(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9,);
tuple_multi_from_str_impl!(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10,);
tuple_multi_from_str_impl!(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11,);
