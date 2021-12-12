#![allow(unused_imports)]
use std::collections::{HashMap, HashSet};
use std::mem::size_of;

use helpers::itertools::Itertools;
use helpers::*;

type Input = Vec<(String, String)>;

fn parser() -> Input {
	let re = regex::Regex::new(r"([A-Za-z]+)").unwrap();
	read_stdin()
		.unwrap()
		.trim()
		.lines()
		.map(|l| re_parse(&re, l).unwrap())
		.collect()
}

fn main() {
	let inp = parser();
	let map: HashMap<&str, Vec<&str>> =
		inp.iter()
			.fold(HashMap::with_capacity(inp.len()), |mut map, (a, b)| {
				if a != "start" && b != "end" {
					map.entry(b.as_str()).or_default().push(a.as_str());
				}
				if b != "start" && a != "end" {
					map.entry(a.as_str()).or_default().push(b.as_str());
				}
				map
			});

	// Part 1
	let mut paths = vec![vec!["start"]];
	let mut finished_paths = 0;
	while let Some(path) = paths.pop() {
		let new_paths = map[path.last().unwrap()].citer().flat_map(|next| {
			if next.chars().next().unwrap().is_lowercase() && path.contains(&next) {
				return None;
			}
			let mut new_path = path.clone();
			new_path.push(next);
			if next == "end" {
				finished_paths += 1;
				None
			} else {
				Some(new_path)
			}
		});
		paths.extend(new_paths);
	}
	display(finished_paths);

	// Part 2
	let mut paths = vec![(vec!["start"], false)];
	let mut finished_paths = 0;
	// let mut max_size = 0;
	while let Some((path, small)) = paths.pop() {
		let new_paths = map[path.last().unwrap()].citer().flat_map(|next| {
			let new_small = if next.chars().next().unwrap().is_lowercase() && path.contains(&next) {
				if small {
					return None;
				} else {
					true
				}
			} else {
				small
			};
			let mut new_path = (path.clone(), new_small);
			new_path.0.push(next);
			if next == "end" {
				finished_paths += 1;
				None
			} else {
				Some(new_path)
			}
		});
		paths.extend(new_paths);
		// max_size = max_size.max(paths.iter().map(|path| path.0.len()).sum());
	}
	display(finished_paths);
	// display(max_size);
	// display(max_size * size_of::<&str>() + size_of::<bool>());
}
