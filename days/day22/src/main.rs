#![feature(array_chunks)]
#![feature(array_from_fn)]
#![allow(unused_imports)]

use std::array;

use helpers::*;

const DIM: usize = 3;

type Input = Vec<(bool, [[i64; 2]; DIM])>;

fn parser() -> Input {
	let re = regex::Regex::new(r"([\d-]+)").unwrap();
	read_stdin()
		.unwrap()
		.trim()
		.lines()
		.map(|l| {
			let on = l.starts_with("on");
			let mut dims = l.split(',').map(|d| re_parse(&re, d).unwrap());
			let v = (on, array::try_from_fn(|_| dims.next()).unwrap());
			assert!(dims.next().is_none());
			v
		})
		.collect()
}

fn main() {
	let mut inp = parser().into_iter();

	let mut cubes = Cubes::new();

	// Part 1
	'a: for (state, mut step) in &mut inp {
		for [a, b] in step.iter_mut() {
			*b += 1;

			if !(-50..=50).contains(a) || !(-49..=51).contains(b) {
				display(cubes.len());
				cubes.step(step, state);
				break 'a;
			}
		}
		cubes.step(step, state);
	}

	// Part 2
	for (state, mut step) in &mut inp {
		for [_, b] in step.iter_mut() {
			*b += 1;
		}
		cubes.step(step, state);
	}

	display(cubes.len());
}

#[derive(Debug, Default, Clone)]
struct Cubes {
	map: Vec<[[i64; 2]; DIM]>,
	intersections: Vec<[[i64; 2]; DIM]>,
}

impl Cubes {
	fn new() -> Self {
		Default::default()
	}

	fn len(&self) -> u64 {
		(self.map.citer().fold(0, |acc, c| {
			acc + c.into_iter().map(|[a, b]| b - a).product::<i64>() as u64
		}) - self.intersections.citer().fold(0, |acc, c| {
			acc + c.into_iter().map(|[a, b]| b - a).product::<i64>() as u64
		})) as u64
	}

	fn step(&mut self, step: [[i64; 2]; DIM], state: bool) {
		let mut new_ints = Vec::new();

		for existing in self.map.citer() {
			let intersection = intersect(step, existing);
			if let Some(int) = intersection {
				new_ints.push(int);
			}
		}
		for existing in self.intersections.citer() {
			let intersection = intersect(step, existing);
			if let Some(int) = intersection {
				self.map.push(int);
			}
		}

		self.intersections.extend_from_slice(&new_ints);

		if state {
			self.map.push(step);
		}
	}
}

fn intersect(step: [[i64; 2]; DIM], existing: [[i64; 2]; DIM]) -> Option<[[i64; 2]; DIM]> {
	let mut int = step
		.into_iter()
		.zip(existing)
		.map(|([a1, b1], [a2, b2])| [a1.max(a2), b1.min(b2)]);

	array::try_from_fn(|_| {
		let [a, b] = int.next().unwrap();
		if a >= b {
			None
		} else {
			Some([a, b])
		}
	})
}
