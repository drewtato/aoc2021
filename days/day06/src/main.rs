#![allow(unused_imports)]
use std::{collections::HashMap, iter::repeat};

use helpers::*;

type Number = u64;
type Input = Vec<Number>;

fn parser() -> Input {
	read_stdin()
		.unwrap()
		.trim()
		.split(',')
		.multi_parse()
		.unwrap()
}

fn main() {
	let fishes = parser();

	let fishes = fishes.into_iter().fold([0; 9], |mut acc, fish| {
		acc[fish as usize] += 1;
		acc
	});

	count_fishes(fishes, 80);

	count_fishes(fishes, 256);
}

fn count_fishes(mut fishes: [Number; 9], days: usize) {
	for _day in 0..days {
		let new_fish = fishes[0];
		fishes.copy_within(1..9, 0);
		fishes[6] += new_fish;
		fishes[8] = new_fish;
		display(fishes.into_iter().sum::<Number>());
	}
	display(fishes.into_iter().sum::<Number>());
}
