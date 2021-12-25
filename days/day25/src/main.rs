use std::collections::HashMap;
// use std::fs::create_dir_all;

// use helpers::image::RgbImage;
use helpers::*;

type Input = Vec<Vec<u8>>;

const EAST: u8 = 1;
const SOUTH: u8 = 2;

fn parser() -> Input {
	read_stdin()
		.unwrap()
		.trim()
		.lines()
		.map(|l| {
			l.trim()
				.chars()
				.map(|c| match c {
					'>' => EAST,
					'v' => SOUTH,
					'.' => 0,
					_ => unreachable!(),
				})
				.collect()
		})
		.collect()
}

fn main() {
	let inp = parser();
	let xmax = inp[0].len();
	let ymax = inp.len();

	let mut cucumbers: HashMap<[usize; 2], u8> = inp
		.iter()
		.enumerate()
		.flat_map(|(y, row)| row.iter().enumerate().map(move |(x, &c)| ([y, x], c)))
		.filter(|(_, c)| c > &0)
		.collect();

	// debug(inp);

	// save_img(0, &cucumbers, xmax, ymax);

	for n in 1.. {
		let mut new_map = HashMap::new();
		let mut changed = false;

		for (&[y, x], &cucumber) in &cucumbers {
			// debug(([y, x], cucumber));
			let new_x = (x + 1) % xmax;
			if cucumber == EAST {
				if !cucumbers.contains_key(&[y, new_x]) {
					new_map.insert([y, new_x], cucumber);
					changed = true;
				} else {
					new_map.insert([y, x], cucumber);
				}
			} else {
				new_map.insert([y, x], cucumber);
			}
		}
		cucumbers = new_map;
		let mut new_map = HashMap::new();
		for (&[y, x], &cucumber) in &cucumbers {
			// debug(([y, x], cucumber));
			let new_y = (y + 1) % ymax;
			if cucumber == SOUTH {
				if !cucumbers.contains_key(&[new_y, x]) {
					new_map.insert([new_y, x], cucumber);
					changed = true;
				} else {
					new_map.insert([y, x], cucumber);
				}
			} else {
				new_map.insert([y, x], cucumber);
			}
		}

		cucumbers = new_map;

		// save_img(n, &cucumbers, xmax, ymax);

		if !changed {
			display(n);
			display("Merry Christmas 🎄");
			break;
		}
	}
}

// fn save_img(seq: usize, cucumbers: &HashMap<[usize; 2], u8>, xmax: usize, ymax: usize) {
// 	create_dir_all("./visualizations/day25").unwrap();

// 	let mut img = RgbImage::new(xmax as u32, ymax as u32);
// 	for (x, y, pix) in img.enumerate_pixels_mut() {
// 		*pix = match cucumbers.get(&[y as usize, x as usize]) {
// 			Some(&SOUTH) => [0xFF, 0xFF, 0x66],
// 			Some(&EAST) => [0x99, 0x99, 0xCC],
// 			None => [0x0F, 0x0F, 0x23],
// 			_ => unreachable!(),
// 		}
// 		.into()
// 	}
// 	img.save(format!("./visualizations/day25/{seq:05}.png"))
// 		.unwrap();
// }
