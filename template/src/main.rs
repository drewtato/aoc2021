#![allow(unused_imports)]
use helpers::{self::*, regex::Regex};

type Input = Vec<Vec<i32>>;

fn parser() -> Input {
	let re = Regex::new(r"(\d+)").unwrap();
	read_stdin()
		.unwrap()
		.trim()
		.lines()
		.map(|l| re_parse(&re, l).unwrap())
		.collect()
}

fn main() {
	let inp = parser();
	debug(inp);
}
