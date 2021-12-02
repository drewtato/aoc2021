use std::ops::Deref;

use helpers::{display, read_stdin, TupleParse};

type Input = Vec<(String, i32)>;

fn parser() -> Input {
	read_stdin()
		.unwrap()
		.trim()
		.lines()
		.map(|l| l.split_ascii_whitespace().tuple_parse().unwrap())
		.collect()
}

fn main() {
	let inp = parser();

	// Part 1
	// This was my original implementation. Ugly.
	// let x: i32 = inp
	// 	.iter()
	// 	.filter(|&(s, _)| s == "forward")
	// 	.map(|(_, n)| *n)
	// 	.sum();
	// let y: i32 = inp
	// 	.iter()
	// 	.filter(|&(s, _)| s == "down")
	// 	.map(|(_, n)| *n)
	// 	.sum();
	// let z: i32 = inp
	// 	.iter()
	// 	.filter(|&(s, _)| s == "up")
	// 	.map(|(_, n)| *n)
	// 	.sum();

	// This is my better implementation. Beauty.
	let (x, y, z) = inp
		.iter()
		.fold((0, 0, 0), |(x, y, z), (s, n)| match s.deref() {
			"forward" => (x + n, y, z),
			"down" => (x, y + n, z),
			"up" => (x, y, z + n),
			_ => panic!(),
		});

	display(x * (y - z));

	// Part 2
	let (_aim, depth, horiz) = inp
		.into_iter()
		.fold((0, 0, 0), |(aim, depth, horiz), (ins, n)| {
			match ins.deref() {
				"forward" => (aim, depth + aim * n, horiz + n),
				"down" => (aim + n, depth, horiz),
				"up" => (aim - n, depth, horiz),
				_ => panic!(),
			}
		});

	display(depth * horiz);
}
