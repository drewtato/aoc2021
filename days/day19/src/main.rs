#![allow(unused_imports)]
use std::collections::{HashMap, HashSet};

use helpers::itertools::Itertools;
use helpers::*;

const DIMENSIONS: usize = 3;

type Point = [i32; DIMENSIONS];

type Input = Vec<(usize, Vec<Point>)>;

fn parser() -> Input {
	let title = regex::Regex::new(r"--- scanner ([\d-]+) ---").unwrap();
	let re = regex::Regex::new(r"([\d-]+)").unwrap();
	let mut input = Vec::new();
	for line in read_stdin()
		.unwrap()
		.trim()
		.lines()
		.map(|l| l.trim())
		.filter(|l| !l.is_empty())
	{
		if let Ok([scanner_num]) = re_parse::<[usize; 1]>(&title, line) {
			input.push((scanner_num, Vec::new()));
		} else if let Ok(coords) = re_parse(&re, line) {
			input.last_mut().unwrap().1.push(coords);
		}
	}
	input
}

fn main() {
	let mut inp = parser();
	let len = inp.len();
	// debug(inp);

	let (num, first) = inp.pop().unwrap();

	// let mut all = HashMap::<Point, Vec<usize>>::new();
	// for orientation in orientations() {
	// 	all.entry(orient_point([1, 2, 3], orientation))
	// 		.or_default()
	// 		.push(orientation);
	// }
	// for p in all {
	// 	debug(p);
	// }
	// return;

	let mut beacon_positions: HashMap<usize, HashSet<_>> =
		HashMap::from([(num, first.into_iter().collect())]);

	let mut scanner_positions: HashMap<usize, Point> = HashMap::from([(num, [0; DIMENSIONS])]);

	let mut inp = inp
		.into_iter()
		.map(|(num, scanner)| {
			(
				num,
				orientations()
					.map(|orientation| {
						let mut v = Vec::with_capacity(scanner.len());
						v.extend(
							scanner
								.citer()
								.map(|point| orient_point(point, orientation)),
						);
						v
					})
					.collect_vec(),
			)
		})
		.collect_vec();

	'a: while !inp.is_empty() {
		for guess in 0..25 {
			for fixed_scanner in beacon_positions.values() {
				for (i2, (_, scanner)) in inp.iter().enumerate() {
					for (orientation, oriented_points) in scanner.iter().enumerate() {
						let possible_differences = fixed_scanner
							.citer()
							.map(|p1| sub(p1, oriented_points[guess]));

						for difference in possible_differences {
							let count = oriented_points
								.citer()
								.filter(|&p2| fixed_scanner.contains(&add(p2, difference)))
								.count();

							if count >= 12 {
								let (scanner_num, mut all_oriented) = inp.swap_remove(i2);
								let mut oriented_points = all_oriented.swap_remove(orientation);

								for point in oriented_points.iter_mut() {
									*point = add(*point, difference);
								}
								beacon_positions
									.insert(scanner_num, oriented_points.citer().collect());

								scanner_positions.insert(scanner_num, difference);

								continue 'a;
							}
						}
					}
				}
			}
		}
		debug(beacon_positions.ckeys().collect_vec());
		debug(inp.iter().map(|(a, _)| *a).collect_vec());
		panic!("Did not find match, {} of {} left", inp.len(), len);
	}
	// debug(scanner_positions);

	let beacons: HashSet<Point> =
		beacon_positions
			.values()
			.fold(HashSet::new(), |mut all, some| {
				all.extend(some);
				all
			});

	display(beacons.len());
	// for scanner in beacons.into_iter().sorted_unstable() {
	// 	println!("{}", scanner.into_iter().join(","))
	// }

	let farthest = scanner_positions
		.cvalues()
		.tuple_combinations()
		.map(|(s1, s2)| sub(s1, s2))
		.map(|p| p.into_iter().fold(0, |acc, n| acc + n.abs()))
		.max()
		.unwrap();
	display(farthest);
}

fn add(mut p1: Point, p2: Point) -> Point {
	for (n1, n2) in p1.iter_mut().zip(p2) {
		*n1 += n2;
	}
	p1
}

fn sub(mut p1: Point, p2: Point) -> Point {
	for (n1, n2) in p1.iter_mut().zip(p2) {
		*n1 -= n2;
	}
	p1
}

fn orientations() -> impl Iterator<Item = usize> {
	0..24
}

fn orient_point(mut point: Point, orientation: usize) -> Point {
	let heading = orientation % 6;
	let rotation = orientation / 6;
	let heading_negative = (heading / 3) != 0;
	let heading_direction = heading % 3;

	// debug((orientation, heading_negative, heading_direction, rotation));

	if heading_negative {
		point[0] = -point[0];
		let tmp = point[1];
		point[1] = -point[2];
		point[2] = -tmp;
	}

	match heading_direction {
		1 => {
			let tmp = point[1];
			point[1] = point[0];
			point[0] = -tmp;
		}
		2 => {
			let tmp = point[2];
			point[2] = point[0];
			point[0] = -tmp;
		}
		0 => (),
		_ => unreachable!(),
	}

	match rotation {
		1 => {
			let tmp = point[2];
			point[2] = point[1];
			point[1] = -tmp;
		}
		2 => {
			point[2] = -point[2];
			point[1] = -point[1];
		}
		3 => {
			let tmp = point[1];
			point[1] = point[2];
			point[2] = -tmp;
		}
		0 => (),
		_ => unreachable!(),
	}

	point
}
