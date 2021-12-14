#![feature(array_windows)]
#![allow(unused_imports)]
use std::collections::HashMap;
use std::mem::swap;
use std::slice::SliceIndex;
use std::time::Instant;

use helpers::itertools::Itertools;
use helpers::*;
use rayon::iter::plumbing::Producer;
use rayon::iter::{ParallelBridge, ParallelIterator};

type Input = (Vec<u8>, HashMap<[u8; 2], u8>);

fn parser() -> Input {
	let re = regex::Regex::new(r"(.)(.) -> (.)").unwrap();
	let stdin = read_stdin().unwrap();
	let mut lines = stdin.trim().lines();
	let start = lines.next().unwrap().as_bytes().to_vec();
	lines.next();
	let pairs = lines
		.map(|l| re_parse(&re, l).expect(l))
		.map(|strs: [String; 3]| strs.map(|s| s.as_bytes()[0]))
		.map(|[a, b, c]| ([a, b], c))
		.collect();
	(start, pairs)
}

// These are the max values
// const FIRST: usize = 24;
// const TOTAL: usize = 50;

const FIRST: usize = 19;
const TOTAL: usize = 40;

fn main() {
	let (mut polymer, pair_rules) = parser();
	let mut new_polymer = Vec::with_capacity(polymer.len() * 2);

	// Part 1
	for _ in 0..10 {
		new_polymer.clear();
		for &[a, b] in polymer.array_windows() {
			new_polymer.push(a);
			new_polymer.push(pair_rules[&[a, b]]);
		}
		new_polymer.push(*polymer.last().unwrap());
		swap(&mut polymer, &mut new_polymer);
	}

	let counts = count_polymer(&polymer);

	let (min, max) = counts
		.citer()
		.map(|(_, c)| c)
		.minmax()
		.into_option()
		.unwrap();
	display(max - min);

	// Part 2
	for _ in 0..(FIRST - 10) {
		new_polymer.clear();
		for &[a, b] in polymer.array_windows() {
			new_polymer.push(a);
			new_polymer.push(pair_rules[&[a, b]]);
		}
		new_polymer.push(*polymer.last().unwrap());
		swap(&mut polymer, &mut new_polymer);
	}

	let mut counts: HashMap<u8, _> = count_polymer(&polymer).into_iter().collect();

	let seen_counts = pair_rules
		.ckeys()
		.par_bridge()
		.fold(
			|| (HashMap::new(), Vec::new(), Vec::new()),
			|(mut seen, mut vec1, mut vec2), pair| {
				seen.insert(
					pair,
					pair_insertion(pair, &mut vec1, &mut vec2, &pair_rules),
				);
				(seen, vec1, vec2)
			},
		)
		.map(|(seen, _, _)| seen)
		.reduce(HashMap::new, |seen1, mut seen2| {
			seen2.extend(seen1);
			seen2
		});

	for &pair in polymer.array_windows() {
		// let entry = seen_counts.entry(pair).or_insert_with(|| {
		// 	pair_insertion(pair, &mut int_polymer, &mut new_polymer, &pair_rules)
		// });
		let entry = &seen_counts[&pair];

		for &(p, n) in entry {
			*counts.get_mut(&p).unwrap() += n;
		}
	}

	let (min, max) = counts
		.citer()
		.map(|(_, c)| c)
		.minmax()
		.into_option()
		.unwrap();
	display(max - min);
}

fn pair_insertion(
	pair: [u8; 2],
	int_polymer: &mut Vec<u8>,
	new_polymer: &mut Vec<u8>,
	pair_rules: &HashMap<[u8; 2], u8>,
) -> Vec<(u8, u128)> {
	int_polymer.clear();
	int_polymer.extend(pair);
	for _ in 0..(TOTAL - FIRST) {
		new_polymer.clear();
		for &[a, b] in int_polymer.array_windows() {
			new_polymer.push(a);
			new_polymer.push(pair_rules[&[a, b]]);
		}
		new_polymer.push(*int_polymer.last().unwrap());
		swap(int_polymer, new_polymer);
	}
	count_polymer(&int_polymer[1..(&int_polymer.len() - 1)])
}

fn count_polymer(polymer: &[u8]) -> Vec<(u8, u128)> {
	let counts = polymer.iter().copied().counts();
	counts.citer().map(|(k, v)| (k, v as u128)).collect()
}

#[allow(dead_code)]
fn display_polymer(polymer: &[u8]) {
	display(polymer.iter().map(|&a| a as char).collect::<String>());
}
