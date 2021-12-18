#![allow(unused_imports)]
use std::fmt::Debug;
use std::iter::Sum;
use std::ops::Add;

use helpers::itertools::Itertools;
use helpers::*;

type Input = Vec<Vec<Integer>>;
type Integer = i32;

const OPEN: Integer = -1;
const CLOSE: Integer = -2;

fn parser() -> Input {
	read_stdin()
		.unwrap()
		.trim()
		.lines()
		.map(|l| {
			l.chars()
				.flat_map(|c| match c {
					'[' => Some(OPEN),
					']' => Some(CLOSE),
					',' => None,
					n => Some(n.to_digit(10).unwrap() as Integer),
				})
				.collect()
		})
		.collect()
}

fn main() {
	let inp = parser();
	// debug(inp);
	// let pairs = inp.iter().map(Vec::as_slice).map(Pair::from_slice);

	// let sum: Pair = pairs.sum();
	// display(sum.magnitude());

	// Part 1
	let sum = inp.clone().into_iter().reduce(add).unwrap();
	display(magnitude(sum));

	// Part 2
	let best = inp
		.into_iter()
		.tuple_combinations()
		.map(|(p1, p2)| {
			[(p1.clone(), p2.clone()), (p2, p1)]
				.into_iter()
				.map(|(a, b)| magnitude(add(a, b)))
				.max()
				.unwrap()
		})
		.max()
		.unwrap();

	display(best);
}

fn add(mut p1: Vec<Integer>, p2: Vec<Integer>) -> Vec<Integer> {
	p1.insert(0, OPEN);
	p1.extend(p2);
	p1.push(CLOSE);
	while explode(&mut p1) || split(&mut p1) {}
	p1
}

fn magnitude(pairs: Vec<Integer>) -> Integer {
	let mut lefts = Vec::new();
	for n in pairs {
		match n {
			OPEN => (),
			CLOSE => {
				let a = lefts.pop().unwrap();
				let b = lefts.pop().unwrap();
				lefts.push(a * 2 + b * 3);
			}
			n => lefts.push(n),
		}
	}
	assert_eq!(lefts.len(), 1);
	lefts[0]
}

fn split(pairs: &mut Vec<Integer>) -> bool {
	for (i, p) in pairs.iter_mut().enumerate() {
		if *p >= 10 {
			let big = *p;
			pairs.splice(i..=i, [OPEN, big / 2, (big + 1) / 2, CLOSE]);
			return true;
		}
	}
	false
}

fn explode(pairs: &mut Vec<Integer>) -> bool {
	let mut depth = 0;
	for (i, p) in pairs.iter_mut().enumerate() {
		match *p {
			OPEN => {
				depth += 1;
				if depth > 4 {
					let (_, a, b, _) = pairs.splice(i..(i + 4), [0]).next_tuple().unwrap();
					for n in pairs[..i].iter_mut().rev() {
						if *n >= 0 {
							*n += a;
							break;
						}
					}
					for n in pairs[(i + 1)..].iter_mut() {
						if *n >= 0 {
							*n += b;
							break;
						}
					}
					return true;
				}
			}
			CLOSE => depth -= 1,
			_ => (),
		}
	}
	false
}

// #[derive(Clone, Hash, PartialEq, Eq)]
// struct Pair {
// 	left: PairElement,
// 	right: PairElement,
// }

// #[derive(Clone, Hash, PartialEq, Eq)]
// enum PairElement {
// 	Another(Box<Pair>),
// 	Num(Integer),
// }
// use PairElement::*;

// impl Pair {
// 	fn from_slice(line: &[Integer]) -> Pair {
// 		// dbg!(line);
// 		let inner = &line[1..(line.len() - 1)];
// 		if inner.len() == 2 {
// 			// dbg!("Two num");
// 			Pair {
// 				left: Num(inner[0]),
// 				right: Num(inner[1]),
// 			}
// 		} else if inner[0] != OPEN {
// 			// dbg!("First num");
// 			Pair {
// 				left: Num(inner[0]),
// 				right: PairElement::pair_from_slice(&inner[1..]),
// 			}
// 		} else if *inner.last().unwrap() != CLOSE {
// 			// dbg!("Second num");
// 			Pair {
// 				left: PairElement::pair_from_slice(&inner[..(inner.len() - 1)]),
// 				right: Num(*inner.last().unwrap()),
// 			}
// 		} else {
// 			// dbg!("Two pairs");
// 			let mut nest = 0;
// 			for (i, c) in inner.citer().enumerate() {
// 				match c {
// 					OPEN => nest += 1,
// 					CLOSE => nest -= 1,
// 					_ => (),
// 				}
// 				if nest == 0 {
// 					return Pair {
// 						left: PairElement::pair_from_slice(&inner[..=i]),
// 						right: PairElement::pair_from_slice(&inner[(i + 1)..]),
// 					};
// 				}
// 			}
// 			panic!("Unfinished pair");
// 		}
// 	}

// 	fn into_reduced(mut self) -> Pair {
// 		while self.explode() || self.split() {}
// 		self
// 	}

// 	fn magnitude(&self) -> u64 {
// 		todo!()
// 	}

// 	fn explode(&mut self) -> bool {
// 		self.explode_(0).0
// 	}

// 	fn explode_(&mut self, depth: usize) -> (bool, Option<Integer>, Option<Integer>) {
// 		match (&mut self.left, &mut self.right) {
// 			(Another(p1), Another(p2)) => {
// 				let (exploded, add_left, add_right) = p1.explode_(depth + 1);
// 				if exploded {
// 					todo!()
// 				}
// 				let (exploded, add_left, add_right) = p2.explode_(depth + 1);
// 				if exploded {
// 					if let Some(n) = add_left {
// 						todo!()
// 					}
// 				}
// 			}
// 			(Another(_), Num(_)) => todo!(),
// 			(Num(_), Another(_)) => todo!(),
// 			(Num(_), Num(_)) => todo!(),
// 		}
// 		todo!()
// 	}

// 	fn split(&mut self) -> bool {
// 		for elem in [&mut self.left, &mut self.right] {
// 			match elem {
// 				Another(pair) => {
// 					if pair.split() {
// 						return true;
// 					}
// 				}
// 				&mut Num(n) => {
// 					if n > 10 {
// 						*elem = PairElement::pair(Pair {
// 							left: Num(n / 2),
// 							right: Num((n + 1) / 2),
// 						});
// 						return true;
// 					}
// 				}
// 			};
// 		}
// 		false
// 	}
// }

// impl Debug for Pair {
// 	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// 		f.debug_list().entries([&self.left, &self.right]).finish()
// 	}
// }

// impl Add for Pair {
// 	type Output = Self;

// 	fn add(self, rhs: Self) -> Self::Output {
// 		Pair {
// 			left: PairElement::pair(self),
// 			right: PairElement::pair(rhs),
// 		}
// 		.into_reduced()
// 	}
// }

// impl Sum for Pair {
// 	fn sum<I>(iter: I) -> Self
// 	where
// 		I: Iterator<Item = Self>,
// 	{
// 		iter.reduce(|a, b| a + b).unwrap()
// 	}
// }

// impl Debug for PairElement {
// 	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// 		match self {
// 			Another(pair) => pair.fmt(f),
// 			&Num(n) => write!(f, "{}", n),
// 		}
// 	}
// }

// impl PairElement {
// 	fn pair(pair: Pair) -> Self {
// 		Another(Box::new(pair))
// 	}

// 	fn pair_from_slice(element: &[Integer]) -> Self {
// 		Self::pair(Pair::from_slice(element))
// 	}
// }
