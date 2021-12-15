use std::num::Wrapping;

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
	let inp = parser();

	// let mut new_inp = Vec::new();
	// for n in 0..5 {
	// 	for row in &inp {
	// 		new_inp.push(Vec::new());
	// 		for m in 0..5 {
	// 			for c in row.citer() {
	// 				new_inp.last_mut().unwrap().push(((c + m + n - 1) % 9) + 1);
	// 			}
	// 		}
	// 	}
	// }
	// let inp = new_inp;

	let inp_ref = &inp;

	// Part 1
	let start = [0usize, 0];
	let ymax = inp.len();
	let xmax = inp[0].len();
	let end = [ymax - 1, xmax - 1];
	let solution = pathfinding::directed::astar::astar(
		&start,
		|&[y, x]| {
			NEIGHBORS.into_iter().flat_map(move |[dy, dx]| {
				let y = (w(y) - ONE + w(dy)).0;
				let x = (w(x) - ONE + w(dx)).0;
				inp_ref
					.get(y)
					.and_then(|row| row.get(x))
					.copied()
					.map(|cost| ([y, x], cost as usize))
			})
		},
		|&[y, x]| end[0] - y + end[1] - x,
		|&node| node == end,
	)
	.unwrap();
	display(solution.1);

	// Part 2
	let end = [ymax * 5 - 1, xmax * 5 - 1];
	let solution = pathfinding::directed::astar::astar(
		&start,
		|&[y, x]| {
			NEIGHBORS.into_iter().flat_map(move |[dy, dx]| {
				let y = (w(y) - ONE + w(dy)).0;
				let (yboard, ysub) = (y / ymax, y % ymax);
				let x = (w(x) - ONE + w(dx)).0;
				let (xboard, xsub) = (x / xmax, x % xmax);
				if yboard >= 5 || xboard >= 5 {
					None
				} else {
					inp_ref
						.get(ysub)
						.and_then(|row| row.get(xsub))
						.copied()
						.map(|cost| ([y, x], ((cost as usize + yboard + xboard - 1) % 9) + 1))
				}
			})
		},
		|&[y, x]| end[0] - y + end[1] - x,
		|&node| node == end,
	)
	.unwrap();
	display(solution.1);
}

const NEIGHBORS: [[usize; 2]; 4] = [[0, 1], [2, 1], [1, 0], [1, 2]];
const ONE: Wrapping<usize> = w(1);
