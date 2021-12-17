#![allow(unused_imports)]
use std::collections::HashMap;

use helpers::itertools::Itertools;
use helpers::*;

type Input = Vec<Line>;

fn parser() -> Input {
	read_stdin()
		.unwrap()
		.trim()
		.lines()
		.map(|l| {
			l.split(" -> ")
				.flat_map(|s| s.split(','))
				.multi_parse()
				.unwrap()
		})
		.map(|[x1, y1, x2, y2]: [isize; 4]| Line { x1, y1, x2, y2 })
		.collect()
}

fn main() {
	let inp = parser();
	let (xmin, xmax) = inp
		.iter()
		.flat_map(|line| [line.x1, line.x2])
		.minmax()
		.into_option()
		.unwrap();
	let (ymin, ymax) = inp
		.iter()
		.flat_map(|line| [line.y1, line.y2])
		.minmax()
		.into_option()
		.unwrap();

	// Part 1
	let mut map: Vec<Vec<u8>> =
		vec![vec![0; (ymax - ymin + 1) as usize]; (xmax - xmin + 1) as usize];
	let mut diagonals = Vec::new();
	for line in inp {
		if line.y1 == line.y2 {
			// println!("{:?} is vertical", (l.x1, l.y1, l.x2, l.y2));
			for x in range_reversible_inclusive(line.x1, line.x2) {
				*map.im(x - xmin).im(line.y1 - ymin) += 1;
			}
		} else if line.x1 == line.x2 {
			// println!("{:?} is horizontal", (l.x1, l.y1, l.x2, l.y2));
			for y in range_reversible_inclusive(line.y1, line.y2) {
				*map.im(line.x1 - xmin).im(y - ymin) += 1;
			}
		} else {
			diagonals.push(line);
		}
	}
	// display_2d_map(&map, ".");
	display(map.iter().flatten().filter(|&&v| v >= 2).count());

	// Part 2
	for line in diagonals {
		for (x, y) in range_reversible_inclusive(line.x1, line.x2)
			.zip(range_reversible_inclusive(line.y1, line.y2))
		{
			map[(x - xmin) as usize][(y - ymin) as usize] += 1;
		}
	}
	// image_2d_map(&map, 0, "day05");
	// display_2d_map(&map, ".");
	display(map.iter().flatten().filter(|&&v| v >= 2).count());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Line {
	x1: isize,
	y1: isize,
	x2: isize,
	y2: isize,
}
