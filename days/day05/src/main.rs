use std::collections::HashMap;

use helpers::{display, range_reversible_inclusive, read_stdin, MultiParse};

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

	// Part 1
	let mut map: HashMap<(isize, isize), isize> = HashMap::new();
	for &line in &inp {
		if line.y1 == line.y2 {
			// println!("{:?} is vertical", (l.x1, l.y1, l.x2, l.y2));
			for x in range_reversible_inclusive(line.x1, line.x2) {
				*map.entry((x, line.y1)).or_default() += 1;
			}
		} else if line.x1 == line.x2 {
			// println!("{:?} is horizontal", (l.x1, l.y1, l.x2, l.y2));
			for y in range_reversible_inclusive(line.y1, line.y2) {
				*map.entry((line.x1, y)).or_default() += 1;
			}
		}
	}
	// display_2d_map(&map, ".");
	display(map.values().filter(|&&v| v >= 2).count());

	// Part 2
	let mut map: HashMap<(isize, isize), isize> = HashMap::new();
	for line in inp {
		if line.y1 == line.y2 {
			for x in range_reversible_inclusive(line.x1, line.x2) {
				*map.entry((x, line.y1)).or_default() += 1;
			}
		} else if line.x1 == line.x2 {
			for y in range_reversible_inclusive(line.y1, line.y2) {
				*map.entry((line.x1, y)).or_default() += 1;
			}
		} else {
			for (x, y) in range_reversible_inclusive(line.x1, line.x2)
				.zip(range_reversible_inclusive(line.y1, line.y2))
			{
				*map.entry((x, y)).or_default() += 1;
			}
		}
	}
	// display_2d_map(&map, ".");
	display(map.values().filter(|&&v| v >= 2).count());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Line {
	x1: isize,
	y1: isize,
	x2: isize,
	y2: isize,
}
