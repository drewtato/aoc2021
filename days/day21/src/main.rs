#![feature(array_from_fn)]
#![feature(map_first_last)]
#![allow(unused_imports)]
use std::collections::{BTreeMap, HashMap};
use std::ops::{Add, RangeInclusive};

use helpers::itertools::Itertools;
use helpers::*;

type Input = Vec<[u8; 2]>;

fn parser() -> Input {
	let re = regex::Regex::new(r"(\d+)").unwrap();
	read_stdin()
		.unwrap()
		.trim()
		.lines()
		.map(|l| re_parse(&re, l).unwrap())
		.collect()
}

fn main() {
	let inp = parser();
	// debug(inp);

	// Part 1
	let [mut p1, mut s1, mut p2, mut s2] = [(inp[0][1] - 1) as u32, 0, (inp[1][1] - 1) as u32, 0];
	let mut die = (1..=100).cycle();
	let mut rolls = 0;
	loop {
		p1 += (&mut die).take(3).sum::<u32>();
		rolls += 3;
		p1 %= 10;
		s1 += p1 + 1;
		if s1 >= 1000 {
			break;
		}
		p2 += (&mut die).take(3).sum::<u32>();
		rolls += 3;
		p2 %= 10;
		s2 += p2 + 1;
		if s2 >= 1000 {
			break;
		}
	}
	display(rolls * s1.min(s2));

	// Part 2
	let [p1, p2] = [inp[0][1] - 1, inp[1][1] - 1];

	const QUANTUM_DIE: RangeInclusive<u8> = 1..=3;
	let possibilities = [QUANTUM_DIE; 3]
		.into_iter()
		.multi_cartesian_product()
		.map(|v| v.into_iter().sum())
		.counts();
	let possibilities = possibilities
		.citer()
		.map(|(a, b)| (a, b as u64))
		.collect_vec();

	let mut states = Map::from([(GameState::new(p1, p2), 1)]);

	let mut p1wins = 0;
	let mut p2wins = 0;

	while let Some(state) = states.pop_first() {
		let (state, games_in_state) = state;
		for (add, mult1) in possibilities.citer() {
			let new_p1 = state.p1 + add;
			if new_p1.score >= 21 {
				p1wins += games_in_state * mult1;
			} else {
				for (add, mult2) in possibilities.citer() {
					let new_p2 = state.p2 + add;
					if new_p2.score >= 21 {
						p2wins += games_in_state * mult1 * mult2;
					} else {
						*states
							.entry(GameState {
								p1: new_p1,
								p2: new_p2,
							})
							.or_default() += games_in_state * mult1 * mult2;
					}
				}
			}
		}
	}

	display(p1wins.max(p2wins));
}

type Map = BTreeMap<GameState, u64>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct GameState {
	p1: Player,
	p2: Player,
}

impl GameState {
	fn new(p1: u8, p2: u8) -> Self {
		Self {
			p1: Player { pos: p1, score: 0 },
			p2: Player { pos: p2, score: 0 },
		}
	}
}

impl PartialOrd for GameState {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for GameState {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.p1
			.score
			.cmp(&other.p1.score)
			.then_with(|| self.p2.score.cmp(&other.p2.score))
			.then_with(|| self.p1.pos.cmp(&other.p1.pos))
			.then_with(|| self.p2.pos.cmp(&other.p2.pos))
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Player {
	pos: u8,
	score: u8,
}

impl Add<u8> for Player {
	type Output = Player;

	fn add(mut self, rhs: u8) -> Self::Output {
		self.pos += rhs;
		self.pos %= 10;
		self.score += self.pos + 1;
		self
	}
}
