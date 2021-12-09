#![allow(unused_imports)]
use std::collections::{HashMap, HashSet};

use helpers::itertools::Itertools;
use helpers::*;

type Input = Vec<([Vec<u8>; 10], [Vec<u8>; 4])>;

fn parser() -> Input {
	let re = regex::Regex::new(r"(\w+)").unwrap();
	read_stdin()
		.unwrap()
		.trim()
		.lines()
		.map(|l| re_parse(&re, l).unwrap())
		.map(|l: Vec<String>| {
			(
				l[..(l.len() - 4)].iter().multi_parse().unwrap(),
				l[(l.len() - 4)..].iter().multi_parse().unwrap(),
			)
		})
		.map(|(first, second): ([String; 10], [String; 4])| {
			(
				first.map(|n| n.chars().map(|c| (c as u8) - b'a').collect()),
				second.map(|n| n.chars().map(|c| (c as u8) - b'a').collect()),
			)
		})
		.collect()
}

fn main() {
	let inp = parser();

	// Part 1
	let digits1478: usize = inp
		.iter()
		.map(|(_, output)| {
			output
				.iter()
				.filter(|&s| [2, 4, 3, 7].contains(&s.len()))
				.count()
		})
		.sum();
	display(digits1478);

	// Part 2 (whew)
	// let segment_counts_reverse =
	// 	reverse_multi_hash_map(&multi_hash_map(COUNTS.into_iter().enumerate()));
	let map_reverse: HashMap<u8, usize> = index_map(&MAP);
	// debug(segment_counts_reverse);
	let total: usize = inp
		.into_iter()
		.map(|(all_nums, output_nums)| {
			let counts = all_nums
				.iter()
				.flatten()
				.copied()
				.counts()
				.into_iter()
				.map(|(k, v)| (k, vec![v]))
				.collect();
			let reverse = reverse_multi_hash_map(&counts);
			// debug(reverse.into_iter().sorted().collect_vec());
			let mut segments = [8; 7];
			segments[4] = reverse[&4][0];
			segments[5] = reverse[&9][0];
			segments[1] = reverse[&6][0];
			segments[2] = all_nums
				.iter()
				.find(|&num| num.len() == 2)
				.unwrap()
				.citer()
				.find(|&seg| seg != segments[5])
				.unwrap();
			segments[0] = all_nums
				.iter()
				.find(|&num| num.len() == 3)
				.unwrap()
				.citer()
				.find(|&seg| seg != segments[2] && seg != segments[5])
				.unwrap();
			segments[3] = all_nums
				.iter()
				.find(|&num| num.len() == 4)
				.unwrap()
				.citer()
				.find(|seg| !segments.contains(seg))
				.unwrap();
			segments[6] = (0..=6).find(|seg| !segments.contains(seg)).unwrap();
			let mut rev_seg = [8; 7];
			for (s, &n) in segments.iter().enumerate() {
				rev_seg[n as usize] = s as u8;
			}
			// debug(&rev_seg);
			// debug(&output_nums);
			// debug(segments);
			output_nums
				.into_iter()
				.map(|num| {
					let map_key = num
						.into_iter()
						.map(|seg| rev_seg[seg as usize])
						.fold(0, |a, b| a | 2u8.pow((6 - b) as u32));
					map_reverse[&map_key]
				})
				.reduce(|a, b| a * 10 + b)
				.unwrap() as usize
			// debug(&output_nums);
			// output_nums
			// 	.into_iter()
			// 	.map(|mut num| {
			// 		for n in num.iter_mut() {
			// 			*n = rev_seg[*n as usize];
			// 		}
			// 		num.sort_unstable();
			// 		// debug(&num);
			// 		num
			// 	})
			// 	.map(|num| match num.len() {
			// 		2 => 1,
			// 		3 => 7,
			// 		4 => 4,
			// 		5 => {
			// 			if num[3] == 4 {
			// 				2
			// 			} else if num[1] == 2 {
			// 				3
			// 			} else {
			// 				5
			// 			}
			// 		}
			// 		6 => {
			// 			if num[2] == 3 {
			// 				6
			// 			} else if num[3] == 3 {
			// 				9
			// 			} else {
			// 				0
			// 			}
			// 		}
			// 		7 => 8,
			// 		_ => unreachable!(),
			// 	})
			// 	.reduce(|a, b| a * 10 + b)
			// 	.unwrap()
		})
		// .inspect(|n| display(n))
		.sum();
	display(total);
}

const MAP: [u8; 10] = [
	0b1110111, // 0
	0b0010010, // 1
	0b1011101, // 2
	0b1011011, // 3
	0b0111010, // 4
	0b1101011, // 5
	0b1101111, // 6
	0b1010010, // 7
	0b1111111, // 8
	0b1111011, // 9
];
// const COUNTS: [u8; 7] = [8, 6, 8, 7, 4, 9, 7];
