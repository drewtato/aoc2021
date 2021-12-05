#![allow(unused_imports)]
use helpers::*;

type Input = Vec<i32>;

fn parser() -> Input {
	read_stdin()
		.unwrap()
		.trim()
		.lines()
		.map(|l| l.parse().unwrap())
		.collect()
}

fn main() {
	let inp = parser();
	debug(inp);
}
