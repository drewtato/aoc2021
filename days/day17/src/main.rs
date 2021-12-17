#![allow(unused_imports)]
use helpers::itertools::Itertools;
use helpers::*;

type Input = [i32; 4];

fn parser() -> Input {
	let re = regex::Regex::new(r"([\d-]+)").unwrap();
	re_parse(&re, read_stdin().unwrap().trim()).unwrap()
}

fn main() {
	let [xmin, xmax, ymin, ymax] = parser();
	// debug([xmin, xmax, ymin, ymax]);

	// Part 1
	// let x = 'l: loop {
	// 	'a: for xstart in 0.. {
	// 		let mut x = xstart;
	// 		let mut pos = 0;
	// 		while pos < xmin {
	// 			if x == 0 {
	// 				continue 'a;
	// 			}
	// 			pos += x;
	// 			x -= 1;
	// 		}
	// 		break 'l xstart;
	// 	}
	// };

	let mut highest = 0;
	let mut best_y = 0;
	'b: for ystart in 0..=(-ymin) {
		let mut pos = 0;
		let mut y = ystart;
		let mut max = 0;
		while pos > ymax {
			max = max.max(pos);
			pos += y;
			y -= 1;
		}
		if pos >= ymin {
			highest = max;
			best_y = ystart;
			continue 'b;
		}
	}
	let y = best_y;
	display(highest);

	// Part 2
	let possibles = (0..=xmax)
		.cartesian_product(ymin..=y)
		.filter(|&(mut x, mut y)| {
			// dbg!(x, y);
			let mut xpos = 0;
			let mut ypos = 0;
			loop {
				xpos += x;
				x = if x == 0 { 0 } else { x - 1 };
				ypos += y;
				y -= 1;
				if xpos >= xmin && ypos <= ymax {
					// dbg!(x, y, xpos, ypos);
					return xpos <= xmax && ypos >= ymin;
				}
				// dbg!(ypos, ymin);
				if (x == 0 && xpos < xmin) || xpos > xmax || ypos < ymin {
					// dbg!(x, y, xpos, ypos);
					return false;
				}
			}
		})
		.count();
	display(possibles);
}
