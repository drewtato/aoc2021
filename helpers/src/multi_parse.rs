use std::array;

use std::str::FromStr;

use std::fmt::Display;

/// Error type for [`multi_parse`](MultiParse::multi_parse).
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

/// Trait that parses iterators into collections.
///
/// This works on any collection that implements [`MultiFromStr`]. These include `Vec`, arrays, and
/// tuples of length 0 to 12, when the item type or types in those collections implement
/// [`FromStr`]. This trait and `MultiFromStr` are analogous to [`str::parse`] and [`FromStr`].
///
/// # Examples
///
/// With a [`Vec`]:
/// ```
/// # use helpers::MultiParse;
/// let vec: Vec<usize> = "1,2,3".split(',').multi_parse().unwrap();
/// assert_eq!(vec, vec![1, 2, 3]);
/// ```
///
/// With an [prim@array]:
/// ```
/// # use helpers::MultiParse;
/// let arr: [usize; 3] = "1,2,3".split(',').multi_parse().unwrap();
/// assert_eq!(arr, [1, 2, 3]);
/// ```
///
/// With a [tuple]:
/// ```
/// # use helpers::MultiParse;
/// let tup: (usize, usize, usize) = "1,2,3".split(',').multi_parse().unwrap();
/// assert_eq!(tup, (1, 2, 3));
/// ```
///
/// Tuples can also have elements with different types:
/// ```
/// # use helpers::MultiParse;
/// let tup: (usize, String, f32) = "1,2,3".split(',').multi_parse().unwrap();
/// assert_eq!(tup, (1, "2".to_string(), 3.0));
/// ```
///
/// And both tuples and arrays can be immediately destructured. Usually if you pass these to (or
/// return them from) a function at some point, their types can be inferred completely:
/// ```
/// # use helpers::MultiParse;
/// let (a, b, c): (char, f64, u8) = "1,2,3".split(',').multi_parse().unwrap();
/// assert!(a == '1' && b == 2.0 && c == 3);
///
/// let [a, b, c]: [i128; 3] = "1,2,3".split(',').multi_parse().unwrap();
/// assert_eq!(a - b + c, 2);
/// ```
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

/// Trait allowing a type to be built from an iterator of [`str`].
///
/// It is best to implement this trait for collections, and then use
/// [`multi_parse`](MultiParse::multi_parse) to invoke it. This is analogous to [`FromStr`] and
/// [`str::parse`].
///
/// # Examples
/// ```
/// # use helpers::MultiFromStr;
/// let tup: (u16, String) = MultiFromStr::multi_from_str("3,hello".split(",")).unwrap();
/// assert_eq!(tup, (3, "hello".to_string()));
/// ```
///
/// Here is the same thing, but using [`multi_parse`](MultiParse::multi_parse):
/// ```
/// # use helpers::MultiParse;
/// let tup: (u16, String) = "3,hello".split(",").multi_parse().unwrap();
/// assert_eq!(tup, (3, "hello".to_string()));
/// ```
pub trait MultiFromStr {
	fn multi_from_str<I, S>(iter: I) -> Result<Self, MultiParseError>
	where
		Self: Sized,
		I: IntoIterator<Item = S>,
		S: AsRef<str>;
}

impl<T: FromStr> MultiFromStr for Vec<T> {
	/// `Vec` can be built with [`multi_parse`](MultiParse::multi_parse).
	///
	/// # Errors
	///
	/// Since `Vec` can be of any length, the only possible error is
	/// [`ParseError`](MultiParseError::ParseError).
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
	/// An array of any length can be built with [`multi_parse`](MultiParse::multi_parse).
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
		    impl<$($i,)*> MultiFromStr for ($($i,)*)
			where
				$($i: FromStr,)*
			{
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
