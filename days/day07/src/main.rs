#![allow(unused_imports)]
use helpers::*;
type Input = Vec<i32>;

fn parser() -> Input {
	read_stdin()
		.unwrap()
		.trim()
		.split(',')
		.multi_parse()
		.unwrap()
}

fn main() {
	let mut inp = parser();

	// Part 1
	inp.sort_unstable();
	let middle = inp[inp.len() / 2];
	let fuel: i32 = inp.iter().map(|&crab| (crab - middle).abs()).sum();
	display(fuel);

	// Part 2
	let max = inp.iter().copied().max().unwrap();
	let mut fuel: i32 = inp.iter().copied().map(triangular).sum();
	for middle in 1..=max {
		let fuel_needed: i32 = inp
			.iter()
			.map(|&crab| (crab - middle).abs())
			.map(triangular)
			.sum();
		fuel = fuel.min(fuel_needed);
	}
	display(fuel);
}

fn triangular(n: i32) -> i32 {
	n * (n + 1) / 2
}
