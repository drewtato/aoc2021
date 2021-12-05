#![feature(array_from_fn)]
// #![feature(hash_drain_filter)]

use std::{
	cmp::Eq,
	collections::HashMap,
	fmt::{Debug, Display, Write},
	fs::create_dir_all,
	hash::Hash,
	io::{stdin, stdout, BufRead, Read, Write as IoWrite},
	iter,
};

use image::GrayImage;
pub use itertools;
use itertools::Itertools;
pub use regex;

pub type BoxErr = Box<dyn std::error::Error>;

pub fn read_stdin() -> Result<String, BoxErr> {
	let mut buf = String::new();
	stdin().lock().read_to_string(&mut buf)?;
	Ok(buf)
}

/// Prompts stdin for input and returns trimmed input.
pub fn input() -> String {
	print!("> ");
	stdout().flush().unwrap();
	let mut buf = String::new();
	stdin().lock().read_line(&mut buf).unwrap();
	buf.trim().to_string()
}

pub fn display<T: Display>(value: T) {
	println!("{}", value);
}

pub fn debug<T: Debug>(value: T) {
	println!("{:?}", value);
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

pub fn display_2d_vec<V>(grid: &[Vec<V>])
where
	V: Display,
{
	let width =
		grid.iter()
			.flatten()
			.map(|v| v.to_string().len())
			.max()
			.unwrap() + 1;
	let mut screen = String::new();
	for y in grid {
		for v in y {
			write!(screen, "{:>width$}", v).unwrap();
		}
		screen.push('\n');
	}
	display(screen);
}

pub fn image_2d_vec(grid: &[Vec<usize>], name: &str) {
	let (vmin, vmax) = grid
		.iter()
		.flatten()
		.copied()
		.minmax()
		.into_option()
		.unwrap();
	let mut img = GrayImage::new(grid[0].len() as u32, grid.len() as u32);
	for (grid_row, img_row) in grid.iter().zip(img.rows_mut()) {
		for (&v, pixel) in grid_row.iter().zip(img_row) {
			let color = (v - vmin) * 255 / vmax;
			pixel.0 = [color as u8];
		}
	}
	create_dir_all("visualizations").unwrap();
	img.save(format!("visualizations/{name}.png")).unwrap();
}

pub fn display_2d_map<V>(map: &HashMap<(isize, isize), V>, default: &str)
where
	V: Display,
{
	let (xmin, xmax) = map.keys().map(|&(x, _)| x).minmax().into_option().unwrap();
	let (ymin, ymax) = map.keys().map(|&(_, y)| y).minmax().into_option().unwrap();
	let width = map.values().map(|v| v.to_string().len()).max().unwrap() + 1;
	let mut screen = String::new();
	for y in ymin..=ymax {
		for x in xmin..=xmax {
			if let Some(v) = map.get(&(x, y)) {
				write!(screen, "{:>width$}", v).unwrap();
			} else {
				write!(screen, "{:>width$}", default).unwrap();
			}
		}
		screen.push('\n');
	}
	display(screen);
}

pub fn image_2d_map(map: &HashMap<(isize, isize), isize>, background: isize, name: &str) {
	let (xmin, xmax) = map.keys().map(|&(x, _)| x).minmax().into_option().unwrap();
	let (ymin, ymax) = map.keys().map(|&(_, y)| y).minmax().into_option().unwrap();
	let (vmin, vmax) = map
		.values()
		.copied()
		.chain(Some(background))
		.minmax()
		.into_option()
		.unwrap();
	let mut img = GrayImage::new((xmax - xmin + 1) as u32, (ymax - ymin + 1) as u32);
	for (y, row) in (ymin..=ymax).zip(img.rows_mut()) {
		for (x, pixel) in (xmin..=xmax).zip(row) {
			let map_value = map.get(&(x, y)).copied().unwrap_or(background);
			let color = (map_value - vmin) * 255 / vmax;
			pixel.0 = [color as u8];
		}
	}
	create_dir_all("visualizations").unwrap();
	img.save(format!("visualizations/{name}.png")).unwrap();
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

mod multi_parse;
pub use multi_parse::{MultiFromStr, MultiParse, MultiParseError};
// mod bi_map;
// pub use bi_map::BiMap;
