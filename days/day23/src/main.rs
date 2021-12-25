#![feature(int_abs_diff)]
#![feature(array_from_fn)]
#![feature(array_windows)]
use std::array;
use std::iter::repeat;

use helpers::itertools::Itertools;
use helpers::*;
use pathfinding::directed::dijkstra::dijkstra;
use pathfinding::num_traits::AsPrimitive;

type Input = Rooms<2>;

type Hallway = [Option<u8>; 11];
type Rooms<const N: usize> = [[Option<u8>; N]; 4];

const A: Option<u8> = Some(0);
const B: Option<u8> = Some(1);
const C: Option<u8> = Some(2);
const D: Option<u8> = Some(3);

fn parser() -> Input {
	let stdin = read_stdin().unwrap();
	let mut lines = stdin.trim().lines();

	let mut first = lines.nth(2).unwrap().chars().filter_map(|c| match c {
		'A' => A,
		'B' => B,
		'C' => C,
		'D' => D,
		_ => None,
	});
	let mut second = lines.next().unwrap().chars().filter_map(|c| match c {
		'A' => A,
		'B' => B,
		'C' => C,
		'D' => D,
		_ => None,
	});

	array::from_fn(|_| [first.next(), second.next()])
}

fn main() {
	let inp = parser();

	// Part 1
	let hallway: Hallway = [None; 11];
	let rooms: Rooms<2> = inp;

	let (_path, cost) = dijkstra(
		&(hallway, rooms),
		successors::<2, false>,
		|(hallway, rooms)| hallway == &[None; 11] && rooms == &success_rooms(),
	)
	.unwrap();

	display(cost);

	// Part 2
	let hallway: Hallway = [None; 11];
	let mut rooms: Rooms<4> = [[None; 4]; 4];

	for (i, room) in rooms.iter_mut().enumerate() {
		room[0] = inp[i][0];
		room[1] = MID_ROOMS[i][0];
		room[2] = MID_ROOMS[i][1];
		room[3] = inp[i][1];
	}

	// show_map(&hallway, &rooms);

	let (_path, cost) = dijkstra(
		&(hallway, rooms),
		successors::<4, false>,
		|(hallway, rooms)| hallway == &[None; 11] && rooms == &success_rooms(),
	)
	.unwrap();
	// A* was slower.
	// let (_path, cost) = astar(
	// 	&(hallway, rooms),
	// 	successors::<4, false>,
	// 	heuristic,
	// 	|(_, rooms)| rooms == &success,
	// )
	// .unwrap();

	display(cost);
}

const MID_ROOMS: Rooms<2> = [[D, D], [C, B], [B, A], [A, C]];

const fn success_rooms<const N: usize>() -> Rooms<N> {
	[[Some(0); N], [Some(1); N], [Some(2); N], [Some(3); N]]
}

fn show_map<const N: usize>(hallway: &Hallway, rooms: &Rooms<N>) {
	display(
		hallway
			.iter()
			.map(|c| match c {
				Some(0) => 'A',
				Some(1) => 'B',
				Some(2) => 'C',
				Some(3) => 'D',
				Some(_) => unreachable!(),
				None => '.',
			})
			.collect::<String>(),
	);
	for row in 0..rooms[0].len() {
		display(
			repeat(' ')
				.take(2)
				.chain(Itertools::intersperse(
					rooms.iter().map(|c| match c[row] {
						Some(0) => 'A',
						Some(1) => 'B',
						Some(2) => 'C',
						Some(3) => 'D',
						Some(_) => unreachable!(),
						None => '.',
					}),
					' ',
				))
				.collect::<String>(),
		);
	}
	display("");
	// input::<String>();
}

// fn heuristic<const N: usize>((hallway, rooms): &(Hallway, Rooms<N>)) -> u32 {
// 	let mut cost = 0;
// 	for (x, room) in rooms.iter().enumerate() {
// 		let mut blocked = false;
// 		for (y, cell) in room.iter().enumerate() {
// 			let y = y as u32;
// 			if let &Some(amph) = cell {
// 				if !blocked && amph as usize == x {
// 					continue;
// 				}
// 				blocked = true;
// 				cost += cost_to_move(amph) * (2 + y);
// 				cost += cost_to_move(x) * (2 + y);
// 			} else {
// 				cost += cost_to_move(x) * (2 + y);
// 			}
// 		}
// 	}

// 	for end in [hallway.first().unwrap(), hallway.last().unwrap()] {
// 		if let &Some(amph) = end {
// 			cost += cost_to_move(amph);
// 		}
// 	}

// 	cost
// }

fn successors<const N: usize, const DEBUG: bool>(
	(hallway, rooms): &(Hallway, Rooms<N>),
) -> impl IntoIterator<Item = ((Hallway, Rooms<N>), u32)> {
	// show_map(hallway, rooms);
	let mut succ = vec![];
	for (i, cell) in hallway.iter().enumerate() {
		if let &Some(amph) = cell {
			let amph = amph as usize;
			let dest = (amph + 1) * 2;
			if rooms[amph][0].is_none()
				&& rooms[amph]
					.iter()
					.all(|c| c.is_none() || c.unwrap() as usize == amph)
				&& (((i as isize + 1).min(dest as isize))..=((i as isize - 1).max(dest as isize)))
					.all(|n| hallway[n as usize].is_none())
			{
				if DEBUG {
					display("Instant");
					show_map(hallway, rooms);
				}
				return vec![(
					(del_hall(hallway, i), add_rooms(rooms, [amph, 0], amph)),
					cost_to_move(amph) * (1 + i.abs_diff(dest) as u32),
				)];
			}
		}
	}

	for (x, room) in rooms.iter().enumerate() {
		for (y, &two) in room.array_windows().enumerate() {
			let buried = room
				.iter()
				.skip(y + 2)
				.any(|c| c.is_some() && c != &Some(x as u8));
			match two {
				[None, Some(b)] if b as usize != x || buried => {
					if DEBUG {
						display("Instant");
						show_map(hallway, rooms);
					}
					return vec![(
						(*hallway, move_rooms(rooms, [x, y + 1], [x, y])),
						cost_to_move(b),
					)];
				}
				[Some(a), None] if a as usize == x && !buried => {
					if DEBUG {
						display("Instant");
						show_map(hallway, rooms);
					}
					return vec![(
						(*hallway, move_rooms(rooms, [x, y], [x, y + 1])),
						cost_to_move(a),
					)];
				}
				_ => (),
			}
		}

		if let Some(a) = room[0] {
			let a = a as usize;
			if a != x || room.iter().any(|c| !(c.is_none() || c == &Some(x as u8))) {
				let hall_index = (x + 1) * 2;
				for left in (0..hall_index).rev() {
					if is_next_to_room(left) {
						continue;
					}
					if hallway[left].is_some() {
						break;
					}
					succ.push((
						(add_hall(hallway, left, a), del_rooms(rooms, [x, 0])),
						cost_to_move(a) * (1 + (hall_index - left)) as u32,
					));
				}
				for right in (hall_index + 1)..hallway.len() {
					if is_next_to_room(right) {
						continue;
					}
					if hallway[right].is_some() {
						break;
					}
					succ.push((
						(add_hall(hallway, right, a), del_rooms(rooms, [x, 0])),
						cost_to_move(a) * (1 + (right - hall_index)) as u32,
					));
				}
			}
		}
	}
	if DEBUG {
		display("Start");
		show_map(hallway, rooms);
		for &((h, r), c) in &succ {
			display(c);
			show_map(&h, &r);
		}
	}
	succ
}

fn is_next_to_room(hall_index: usize) -> bool {
	[
		false, false, true, false, true, false, true, false, true, false, false,
	][hall_index]
}

fn move_rooms<const N: usize>(
	rooms: &Rooms<N>,
	[from1, from2]: [usize; 2],
	[to1, to2]: [usize; 2],
) -> Rooms<N> {
	let mut rooms = *rooms;
	rooms[to1][to2] = rooms[from1][from2];
	rooms[from1][from2] = None;
	rooms
}

fn del_hall(hallway: &Hallway, from: usize) -> Hallway {
	let mut hallway = *hallway;
	hallway[from] = None;
	hallway
}

fn del_rooms<const N: usize>(rooms: &Rooms<N>, [from1, from2]: [usize; 2]) -> Rooms<N> {
	let mut rooms = *rooms;
	rooms[from1][from2] = None;
	rooms
}

fn add_hall<A>(hallway: &Hallway, to: usize, amph: A) -> Hallway
where
	A: AsPrimitive<u8>,
{
	let mut hallway = *hallway;
	hallway[to] = Some(amph.as_());
	hallway
}

fn add_rooms<const N: usize, A>(rooms: &Rooms<N>, [to1, to2]: [usize; 2], amph: A) -> Rooms<N>
where
	A: AsPrimitive<u8>,
{
	let mut rooms = *rooms;
	rooms[to1][to2] = Some(amph.as_());
	rooms
}

fn cost_to_move<C>(amph: C) -> u32
where
	C: AsPrimitive<usize>,
{
	match amph.as_() {
		0 => 1,
		1 => 10,
		2 => 100,
		3 => 1000,
		_ => unreachable!(),
	}
}
