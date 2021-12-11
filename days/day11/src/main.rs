#![allow(unused_imports)]
use std::convert::identity;
use std::iter::repeat;

use helpers::itertools::Itertools;
use helpers::*;

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

fn main() {
	let mut inp = parser();
	// debug(inp);
	let mut flashes = 0;
	let mut sync = 0;
	let num_octo = inp[0].len() * inp.len();
	for s in 0.. {
		for row in inp.iter_mut() {
			for octo in row.iter_mut() {
				*octo += 1;
			}
		}
		while inp
			.iter()
			.map(|row| row.iter().any(|&n| n > 9))
			.any(identity)
		{
			let glows: Vec<[usize; 2]> = inp
				.iter_mut()
				.enumerate()
				.flat_map(|(y, row)| {
					repeat(y).zip(row.iter_mut().enumerate().filter(|(_, n)| **n > 9))
				})
				.map(|(y, (x, n))| {
					*n = 0;
					[y, x]
				})
				.collect_vec();
			for [y, x] in glows {
				let neighbors = [
					[0, 0],
					[1, 0],
					[2, 0],
					[0, 1],
					[2, 1],
					[0, 2],
					[1, 2],
					[2, 2],
				];
				for [dy, dx] in neighbors {
					if let Some(neighbor) = inp
						.get_mut((dy + y).wrapping_sub(1))
						.and_then(|row| row.get_mut((dx + x).wrapping_sub(1)))
					{
						if *neighbor != 0 {
							*neighbor += 1;
						}
					}
				}
			}
		}
		// octo_2d_vec(
		// 	&inp.iter()
		// 		.map(|row| row.iter().copied().collect_vec())
		// 		.collect::<Vec<_>>(),
		// 	s,
		// );
		let new_flashes = inp
			.iter()
			.map(|row| row.iter().copied().filter(|&n| n == 0).count())
			.sum::<usize>();
		if s <= 100 {
			flashes += new_flashes;
		}
		if new_flashes == num_octo {
			sync = s + 1;
			break;
		}
	}
	display(flashes);
	display(sync);
}

// fn octo_2d_vec(grid: &[Vec<u8>], seq: usize) {
// 	let mut img = RgbImage::new(grid[0].len() as u32, grid.len() as u32);
// 	for (grid_row, img_row) in grid.iter().zip(img.rows_mut()) {
// 		for (&v, pixel) in grid_row.iter().zip(img_row) {
// 			pixel.0 = if v == 0 { LIGHT_BLUE } else { darkness(v) };
// 		}
// 	}
// 	create_dir_all("visualizations/day11").unwrap();
// 	img.save(format!("visualizations/day11/{seq:03}.png"))
// 		.unwrap();
// }

// fn darkness(darkness: u8) -> [u8; 3] {
// 	let ratio = darkness as f32 / 10.0;
// 	let ratio = (ratio + 1.0) / 2.0;
// 	[
// 		(LIGHT_BLUE[0] as f32 * (1.0 - ratio) + BACKGROUND_BLUE[0] as f32 * ratio) as u8,
// 		(LIGHT_BLUE[1] as f32 * (1.0 - ratio) + BACKGROUND_BLUE[1] as f32 * ratio) as u8,
// 		(LIGHT_BLUE[2] as f32 * (1.0 - ratio) + BACKGROUND_BLUE[2] as f32 * ratio) as u8,
// 	]
// }

// const LIGHT_BLUE: [u8; 3] = [0x89, 0x9C, 0xB2];
// const BACKGROUND_BLUE: [u8; 3] = [0x10, 0x10, 0x19];
