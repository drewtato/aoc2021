use std::collections::HashMap;
// use std::time::Instant;

use helpers::{display, itertools::Itertools, read_stdin, CopyIter, MultiParse};

type Number = u16;
type Input = (Vec<Number>, Vec<Vec<Vec<Number>>>);

struct Board {
	won: bool,
	positions: HashMap<Number, (usize, usize)>,
	rows: Vec<usize>,
	cols: Vec<usize>,
}

impl Board {
	fn new(board: Vec<Vec<Number>>) -> Self {
		let size = board[0].len();
		Board {
			won: false,
			positions: board
				.into_iter()
				.enumerate()
				.flat_map(|(y, row)| row.into_iter().enumerate().map(move |(x, n)| (n, (y, x))))
				.collect::<HashMap<Number, (usize, usize)>>(),
			rows: vec![0; size],
			cols: vec![0; size],
		}
	}
}

fn parser() -> Input {
	let input = read_stdin().unwrap();
	let first_line = input
		.trim()
		.lines()
		.next()
		.unwrap()
		.trim()
		.split(',')
		.multi_parse()
		.unwrap();
	let boards = input
		.trim()
		.split("\n\n")
		.skip(1)
		.map(|block| {
			block
				.lines()
				.map(|l| l.trim().split_ascii_whitespace().multi_parse().unwrap())
				.collect()
		})
		.collect();
	(first_line, boards)
}

fn main() {
	let (order, boards) = parser();
	let size = boards[0][0].len();
	// let start = Instant::now();
	let mut boards = boards.into_iter().map(Board::new).collect_vec();

	// Part 1 and 2
	let mut last_win = None;
	for n in order.citer() {
		for board in boards.iter_mut() {
			let Board {
				won,
				positions,
				rows,
				cols,
			} = board;

			if let Some((y, x)) = positions.remove(&n) {
				rows[y] += 1;
				cols[x] += 1;
				if cols[x] == size || rows[y] == size {
					*won = true;
					if last_win.is_some() {
						last_win = Some(score(positions, n))
					} else {
						let new_score = score(positions, n);
						// display(start.elapsed().as_secs_f32());
						display(new_score);
						last_win = Some(new_score);
					}
				}
			}
		}
		boards.retain(|board| !board.won);
	}
	// display(start.elapsed().as_secs_f32());
	display(last_win.unwrap());
}

fn score(positions: &HashMap<Number, (usize, usize)>, n: Number) -> usize {
	positions.keys().copied().map(|n| n as usize).sum::<usize>() * n as usize
}
