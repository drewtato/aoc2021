#![allow(unused_imports)]
use std::collections::HashSet;
use std::ops::Add;

use helpers::itertools::Itertools;
use helpers::*;
use image::GrayImage;

type Input = Vec<Vec<u8>>;

fn parser() -> Input {
	let re = regex::Regex::new(r"(\d)").unwrap();
	read_stdin()
		.unwrap()
		.trim()
		.lines()
		.map(|l| re_parse(&re, l).unwrap())
		.collect()
}

const NEIGHBORS: [[usize; 2]; 4] = [[0, 1], [2, 1], [1, 0], [1, 2]];

fn main() {
	let inp = parser();
	// debug(inp);

	// Part 1
	let mut risk_level = 0;
	let mut low_points = Vec::new();
	for (y, row) in inp.iter().enumerate() {
		for (x, &point) in row.iter().enumerate() {
			let least_neigbor = NEIGHBORS
				.into_iter()
				.flat_map(|[dy, dx]| {
					inp.get((y + dy).wrapping_sub(1))
						.and_then(|row| row.get((x + dx).wrapping_sub(1)))
				})
				.copied()
				.min()
				.unwrap();
			if least_neigbor > point {
				risk_level += point as usize + 1;
				low_points.push([y, x]);
			}
		}
	}
	display(risk_level);

	// Part 2 original method, involved opening in GIMP and seeing how big selections were.
	// image_2d_vec(
	// 	&inp.into_iter()
	// 		.map(|row| {
	// 			row.into_iter()
	// 				.map(|p| if p == 9 { 1 } else { 0 })
	// 				.collect()
	// 		})
	// 		.collect_vec(),
	// 	"day09",
	// );

	// Part 2 done properly
	let sizes = low_points
		.into_iter()
		.map(|start| {
			let mut candidates = vec![start];
			let mut visited = HashSet::new();
			visited.insert(start);
			while let Some([y, x]) = candidates.pop() {
				for [dy, dx] in NEIGHBORS {
					let iy = (y + dy).wrapping_sub(1);
					let ix = (x + dx).wrapping_sub(1);
					let n_value = inp
						.get(iy)
						.and_then(|row| row.get(ix))
						.copied()
						.unwrap_or(9);
					if n_value != 9 && visited.insert([iy, ix]) {
						candidates.push([iy, ix]);
					}
				}
			}
			visited.len() as isize
		})
		.min_n_by_key(3, |&n| -n);
	let biggest_3: isize = sizes.into_iter().product();
	display(biggest_3);
}
