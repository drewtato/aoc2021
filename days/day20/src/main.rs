#![allow(unused_imports)]
use std::collections::HashSet;
use std::io::BufRead;

use helpers::itertools::Itertools;
use helpers::*;

type Map = HashSet<[isize; 2]>;

type Input = (Vec<bool>, Map);

fn parser() -> Input {
	let inp = read_stdin().unwrap();
	let (first, rest) = inp.split_once("\n\n").unwrap();
	let first = first.trim().chars().map(|c| c == '#').collect();
	let rest = rest
		.trim()
		.lines()
		.enumerate()
		.flat_map(|(y, l)| {
			l.trim().chars().enumerate().flat_map(move |(x, c)| {
				if c == '#' {
					Some([y as isize, x as isize])
				} else {
					None
				}
			})
		})
		.collect();
	(first, rest)
}

fn main() {
	let (rule, mut grid) = parser();
	// debug(inp);
	for _ in 0..1 {
		// display_map(&grid);
		grid = enhance(&rule, grid);
		// display_map(&grid);
		if rule[0] {
			grid = enhance2(&rule, grid);
		} else {
			grid = enhance(&rule, grid);
		}
	}
	// display_map(&grid);

	display(count_on(&grid));

	for _ in 0..24 {
		// display_map(&grid);
		grid = enhance(&rule, grid);
		// display_map(&grid);
		if rule[0] {
			grid = enhance2(&rule, grid);
		} else {
			grid = enhance(&rule, grid);
		}
	}
	// display_map(&grid);

	display(count_on(&grid));
}

// fn display_map(grid: &Map) {
// 	display_2d_map(&grid.citer().map(|coord| (coord, "#")).collect(), ".");
// }

fn count_on(grid: &Map) -> usize {
	grid.iter().count()
}

fn enhance(rule: &[bool], grid: Map) -> Map {
	let (ymin, ymax) = grid.citer().map(|[y, _]| y).minmax().into_option().unwrap();
	let (xmin, xmax) = grid.citer().map(|[_, x]| x).minmax().into_option().unwrap();

	let mut new_grid = Map::new();

	let yes = !rule[0];

	for y in (ymin - 1)..=(1 + ymax) {
		for x in (xmin - 1)..=(1 + xmax) {
			let index = REGION
				.into_iter()
				.map(|[dy, dx]| grid.contains(&[dy + y, dx + x]) as usize)
				.fold(0, |acc, v| (acc << 1) + v);
			if rule[index] == yes {
				new_grid.insert([y, x]);
			}
		}
	}

	new_grid
}

fn enhance2(rule: &[bool], grid: Map) -> Map {
	let (ymin, ymax) = grid.citer().map(|[y, _]| y).minmax().into_option().unwrap();
	let (xmin, xmax) = grid.citer().map(|[_, x]| x).minmax().into_option().unwrap();

	let mut new_grid = Map::new();

	let yes = rule[0];

	for y in (ymin - 1)..=(1 + ymax) {
		for x in (xmin - 1)..=(1 + xmax) {
			let index = REGION
				.into_iter()
				.map(|[dy, dx]| !grid.contains(&[dy + y, dx + x]) as usize)
				.fold(0, |acc, v| (acc << 1) + v);
			if rule[index] == yes {
				new_grid.insert([y, x]);
			}
		}
	}

	new_grid
}

const REGION: [[isize; 2]; 9] = [
	[-1, -1],
	[-1, 0],
	[-1, 1],
	[0, -1],
	[0, 0],
	[0, 1],
	[1, -1],
	[1, 0],
	[1, 1],
];
