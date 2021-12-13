#![allow(unused_imports)]
use std::collections::HashSet;
use std::mem::swap;

use helpers::image::EncodableLayout;
use helpers::*;

type Input = (HashSet<[usize; 2]>, Vec<(u8, usize)>);

const Y: u8 = b'y';
const X: u8 = b'x';

fn parser() -> Input {
	let re = regex::Regex::new(r"(\d+)").unwrap();
	let re2 = regex::Regex::new(r"(\w)=(\d+)").unwrap();
	let read = read_stdin().unwrap();
	let (first, second) = read.trim().split_once("\n\n").unwrap();
	let first = first.lines().map(|l| re_parse(&re, l).unwrap()).collect();
	let second = second
		.lines()
		.map(|l| re_parse(&re2, l).unwrap())
		.map(|(c, n): (String, usize)| (c.as_bytes()[0], n))
		.collect();
	(first, second)
}

fn main() {
	let (mut dots, folds) = parser();
	// debug(dots);
	// debug(folds);
	let mut new_dots = HashSet::with_capacity(dots.len());

	// Part 1
	let mut folds = folds.into_iter();
	let (axis, location) = folds.next().unwrap();
	fold_dots(&mut dots, axis, location, &mut new_dots);
	display(dots.len());

	// Part 2
	for (axis, location) in folds {
		fold_dots(&mut dots, axis, location, &mut new_dots);
	}
	display_2d_map(
		&dots
			.iter()
			.map(|&[x, y]| ([x as isize, y as isize], "#"))
			.collect(),
		" ",
	);
}

fn fold_dots(
	dots: &mut HashSet<[usize; 2]>,
	axis: u8,
	location: usize,
	new_dots: &mut HashSet<[usize; 2]>,
) {
	for [x, y] in dots.drain() {
		let new_dot = match axis {
			Y => {
				if y > location {
					[x, location * 2 - y]
				} else {
					[x, y]
				}
			}
			X => {
				if x > location {
					[location * 2 - x, y]
				} else {
					[x, y]
				}
			}
			_ => unreachable!(),
		};
		new_dots.insert(new_dot);
	}
	swap(dots, new_dots);
}
