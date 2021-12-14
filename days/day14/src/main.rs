#![feature(array_windows)]
// #![allow(unused_imports)]
use std::collections::HashMap;

use helpers::itertools::Itertools;
use helpers::*;

type Input = (Vec<u8>, HashMap<[u8; 2], [u8; 3]>);

fn parser() -> Input {
	let stdin = read_stdin().unwrap();
	let mut lines = stdin.trim().lines();
	let start = lines
		.next()
		.unwrap()
		.as_bytes()
		.iter()
		.map(|&b| b - b'A')
		.collect();
	lines.next();
	let pairs = lines
		.map(str::as_bytes)
		.map(|l| [l[0], l[1], l[6]])
		.map(|arr| arr.map(|n| n - b'A'))
		.map(|[a, b, c]| ([a, b], [a, c, b]))
		.collect();
	(start, pairs)
}

type Count = u128;

const EXPANSION: usize = 2;
const MAX_ITERS: usize = 40;

const PART_1_ITERATIONS: usize = 10 / EXPANSION;
const PART_2_ITERATIONS: usize = (MAX_ITERS - 10) / EXPANSION;

#[allow(clippy::assertions_on_constants)]
const _: () = assert!(EXPANSION * PART_1_ITERATIONS == 10);
#[allow(clippy::assertions_on_constants)]
const _: () = assert!(EXPANSION * PART_2_ITERATIONS == MAX_ITERS - 10);

fn main() {
	let (polymer, pair_rules) = parser();

	let pair_counts: HashMap<[u8; 2], HashMap<[u8; 2], Count>> = pair_rules
		.into_iter()
		.map(|(pair, [a, b, c])| (pair, HashMap::from([([a, b], 1), ([b, c], 1)])))
		.collect();

	let mut expanded_pair_counts = pair_counts.clone();
	for _ in 0..(EXPANSION - 1) {
		for counts in expanded_pair_counts.values_mut() {
			*counts = expand_polymer(counts, &pair_counts);
		}
	}

	let mut expanded_polymer = count_elements(&polymer);
	for _ in 0..PART_1_ITERATIONS {
		expanded_polymer = expand_polymer(&expanded_polymer, &expanded_pair_counts);
	}
	display(max_minus_min(&expanded_polymer, polymer[0]));

	for _ in 0..PART_2_ITERATIONS {
		expanded_polymer = expand_polymer(&expanded_polymer, &expanded_pair_counts);
	}
	display(max_minus_min(&expanded_polymer, polymer[0]));
}

fn count_elements(polymer: &[u8]) -> HashMap<[u8; 2], Count> {
	polymer
		.array_windows()
		.copied()
		.counts()
		.into_iter()
		.map(|(k, v)| (k, v as Count))
		.collect()
}

fn max_minus_min(expanded_polymer: &HashMap<[u8; 2], Count>, first: u8) -> Count {
	let elements = expanded_polymer
		.citer()
		.map(|([_, b], n)| (b, n))
		.chain([(first, 1)])
		.fold(
			HashMap::new(),
			|mut counts: HashMap<u8, Count>, (elem, n)| {
				*counts.entry(elem).or_default() += n;
				counts
			},
		);
	let (min, max) = elements.cvalues().minmax().into_option().unwrap();
	max - min
}

fn expand_polymer(
	polymer: &HashMap<[u8; 2], Count>,
	pair_counts: &HashMap<[u8; 2], HashMap<[u8; 2], Count>>,
) -> HashMap<[u8; 2], Count> {
	let mut new_map = HashMap::with_capacity(polymer.len());
	for (pair, &multiplier) in polymer {
		for (pair, count) in pair_counts[pair].citer() {
			*new_map.entry(pair).or_default() += count * multiplier;
		}
	}
	new_map
}
