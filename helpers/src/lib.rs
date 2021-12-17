#![feature(array_from_fn)]
// #![feature(hash_drain_filter)]

use std::{
	cmp::Eq,
	collections::HashMap,
	fmt::{Debug, Display, Write as FmtWrite},
	hash::Hash,
	io::{stdin, stdout, BufRead, Read, Write as IoWrite},
	iter,
	str::FromStr,
};

pub use image;
pub use itertools;
pub use regex;

pub type BoxErr = Box<dyn std::error::Error>;

pub fn read_stdin() -> Result<String, BoxErr> {
	let mut buf = String::new();
	for line in stdin().lock().lines() {
		writeln!(buf, "{}", line?.trim()).unwrap();
	}
	Ok(buf)
}

pub fn read_stdin_fast() -> Result<String, BoxErr> {
	let mut buf = String::new();
	stdin().lock().read_to_string(&mut buf)?;
	Ok(buf)
}

/// Prompts stdin for input and returns a parsed value.
pub fn input<T>() -> T
where
	<T as FromStr>::Err: Debug,
	T: FromStr,
{
	loop {
		print!("> ");
		stdout().flush().unwrap();
		let mut buf = String::new();
		stdin().lock().read_line(&mut buf).unwrap();
		if let Ok(t) = buf.trim().parse() {
			break t;
		}
	}
}

pub use vec_ext::*;
mod vec_ext;

#[allow(clippy::print_with_newline)]
pub fn display<T: Display>(value: T) {
	print!("{}\n", value);
}

#[allow(clippy::print_with_newline)]
pub fn debug<T: Debug>(value: T) {
	print!("{:?}\n", value);
}

pub fn reverse_hash_map<K, V>(map: &HashMap<K, V>) -> HashMap<V, K>
where
	K: Clone,
	V: Clone + Hash + Eq,
{
	map.iter().map(|(k, v)| (v.clone(), k.clone())).collect()
}

pub fn multi_hash_map<K, V, I>(iter: I) -> HashMap<K, Vec<V>>
where
	K: Hash + Eq,
	I: IntoIterator<Item = (K, V)>,
{
	let iter = iter.into_iter();
	let mut map: HashMap<K, Vec<V>> = HashMap::with_capacity(iter.size_hint().0);
	for (k, v) in iter {
		map.entry(k).or_default().push(v);
	}
	map
}

pub fn reverse_multi_hash_map<K, V>(map: &HashMap<K, Vec<V>>) -> HashMap<V, Vec<K>>
where
	K: Clone,
	V: Clone + Hash + Eq,
{
	let mut new_map: HashMap<V, Vec<K>> = HashMap::with_capacity(map.values().map(Vec::len).sum());
	for (k, vs) in map.iter() {
		for v in vs {
			new_map.entry(v.clone()).or_default().push(k.clone())
		}
	}
	new_map
}

pub fn range_reversible(mut start: isize, end: isize) -> impl Iterator<Item = isize> {
	let increment = if start < end { 1 } else { -1 };
	iter::from_fn(move || {
		if start == end {
			return None;
		}
		let v = start;
		start += increment;
		Some(v)
	})
}

pub fn range_reversible_inclusive(start: isize, end: isize) -> impl Iterator<Item = isize> {
	if start < end {
		range_reversible(start, end + 1)
	} else {
		range_reversible(start, end - 1)
	}
}

/// Reverses a `HashMap<K, V>` into a `HashMap<V, K>`, overwriting any duplicates.
pub fn reverse_map_single<K, V>(map: &HashMap<K, V>) -> HashMap<V, K>
where
	K: Clone,
	V: Clone + Eq + Hash,
{
	map.iter().map(|(k, v)| (v.clone(), k.clone())).collect()
}

/// Takes a slice and outputs a `HashMap` of values to indexes.
pub fn index_map<T: Eq + Hash + Clone>(s: &[T]) -> HashMap<T, usize> {
	s.iter().enumerate().map(|(i, t)| (t.clone(), i)).collect()
}

mod visual;
pub use visual::*;

mod multi_parse;
pub use multi_parse::*;

// mod bi_map;
// pub use bi_map::BiMap;

mod re;
pub use re::*;

mod cellauto;
pub use cellauto::*;

mod signed_vec;
pub use signed_vec::*;

mod default_map;
pub use default_map::*;

mod more_itertools;
pub use more_itertools::*;
