#![allow(unused_imports)]
use std::collections::BinaryHeap;

use helpers::*;

type Input = Vec<Vec<u8>>;

fn parser() -> Input {
	read_stdin()
		.unwrap()
		.trim()
		.lines()
		.map(|l| l.trim().as_bytes().to_vec())
		.collect()
}

fn main() {
	let inp: Vec<Vec<u8>> = parser();
	// debug(inp);

	// Part 1
	let mut error_score = 0;
	// Part 2
	let mut autocomplete_scores = BinaryHeap::with_capacity(inp.len());

	// Both parts
	let mut stack = Vec::with_capacity(100);
	'a: for line in inp {
		stack.clear();
		for c in line {
			match c {
				b'(' => stack.push(c),
				b'[' => stack.push(c),
				b'{' => stack.push(c),
				b'<' => stack.push(c),
				_ => {
					let open = stack.pop().unwrap();
					match (c, open) {
						(b'>', b'<') => (),
						(b'}', b'{') => (),
						(b']', b'[') => (),
						(b')', b'(') => (),
						(b')', _) => {
							error_score += 3;
							continue 'a;
						}
						(b']', _) => {
							error_score += 57;
							continue 'a;
						}
						(b'}', _) => {
							error_score += 1197;
							continue 'a;
						}
						(b'>', _) => {
							error_score += 25137;
							continue 'a;
						}
						_ => unreachable!(),
					}
				}
			}
		}

		let auto_score: u64 = stack.iter().rev().fold(0, |score, &c| {
			score * 5
				+ match c {
					b'(' => 1,
					b'[' => 2,
					b'{' => 3,
					b'<' => 4,
					_ => unreachable!(),
				}
		});
		autocomplete_scores.push(auto_score);
	}

	// Part 1
	display(error_score);

	// Part 2
	let median = autocomplete_scores.len() / 2;
	let autocomplete_score = autocomplete_scores.into_sorted_vec()[median];
	display(autocomplete_score);
}
