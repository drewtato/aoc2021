#![feature(int_abs_diff)]
#![feature(array_windows)]
use std::iter::repeat;

use helpers::itertools::Itertools;
use helpers::*;
use pathfinding::directed::dijkstra;
use pathfinding::num_traits::AsPrimitive;

type Input = [[Option<u8>; 11]; 3];

type Hallway = [Option<u8>; 11];
type Rooms<const N: usize> = [[Option<u8>; N]; 4];

fn parser() -> Input {
	let all = read_stdin()
		.unwrap()
		.trim()
		.lines()
		.map(|l| l.chars().collect_vec())
		.collect_vec();

	let mut map = [[None; 11]; 3];
	map[1][2] = Some(all[2][3] as u8);
	map[1][4] = Some(all[2][5] as u8);
	map[1][6] = Some(all[2][7] as u8);
	map[1][8] = Some(all[2][9] as u8);
	map[2][2] = Some(all[3][1] as u8);
	map[2][4] = Some(all[3][3] as u8);
	map[2][6] = Some(all[3][5] as u8);
	map[2][8] = Some(all[3][7] as u8);
	map
}

fn main() {
	let inp = parser();

	// Part 1
	let hallway: Hallway = [None; 11];
	let mut rooms: Rooms<2> = [[None; 2]; 4];

	for (y, row) in inp.iter().skip(1).enumerate() {
		for (x, c) in row.citer().enumerate() {
			if let Some(amph) = c {
				rooms[(x - 2) / 2][y] = Some(amph - b'A');
			}
		}
	}

	let success: Rooms<2> = success_rooms();
	let (_path, cost) =
		dijkstra::dijkstra(&(hallway, rooms), successors::<2, false>, |(_, rooms)| {
			rooms == &success
		})
		.unwrap();

	display(cost);

	// Part 2
	let hallway: Hallway = [None; 11];
	let mut rooms: Rooms<4> = [[None; 4]; 4];

	let mut inp_iter = inp.iter().skip(1);
	let (row1, row2) = inp_iter.next_tuple().unwrap();
	for (y, row) in [(0, row1), (3, row2)] {
		for (x, c) in row.citer().enumerate() {
			if let Some(amph) = c {
				rooms[(x - 2) / 2][y] = Some(amph - b'A');
			}
		}
	}

	for (y, row) in "DCBA\nDBAC".lines().enumerate() {
		for (x, c) in row.as_bytes().citer().enumerate() {
			rooms[x][y + 1] = Some(c - b'A');
		}
	}

	// show_map(&hallway, &rooms);

	let success: Rooms<4> = success_rooms();
	let (_path, cost) =
		dijkstra::dijkstra(&(hallway, rooms), successors::<4, false>, |(_, rooms)| {
			rooms == &success
		})
		.unwrap();

	display(cost);
}

fn success_rooms<const N: usize>() -> Rooms<N> {
	[[Some(0); N], [Some(1); N], [Some(2); N], [Some(3); N]]
}

fn show_map<const N: usize>(hallway: &Hallway, rooms: &Rooms<N>) {
	display(
		hallway
			.citer()
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

fn successors<const N: usize, const DEBUG: bool>(
	(hallway, rooms): &(Hallway, Rooms<N>),
) -> impl IntoIterator<Item = ((Hallway, Rooms<N>), u32)> {
	// show_map(hallway, rooms);
	let mut succ = vec![];
	for (i, cell) in hallway.citer().enumerate() {
		if let Some(amph) = cell {
			let amph = amph as usize;
			let dest = (amph + 1) * 2;
			if rooms[amph][0].is_none()
				&& rooms[amph]
					.citer()
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
				.citer()
				.skip(y + 2)
				.any(|c| c.is_some() && c != Some(x as u8));
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
			if a != x || room.citer().any(|c| !(c.is_none() || c == Some(x as u8))) {
				let hall_index = (x + 1) * 2;
				for left in (0..hall_index).rev() {
					if hallway[left].is_some() {
						break;
					}
					if [2, 4, 6, 8].contains(&left) {
						continue;
					}
					succ.push((
						(add_hall(hallway, left, a), del_rooms(rooms, [x, 0])),
						cost_to_move(a) * (1 + (hall_index - left)) as u32,
					));
				}
				for right in (hall_index + 1)..hallway.len() {
					if hallway[right].is_some() {
						break;
					}
					if [2, 4, 6, 8].contains(&right) {
						continue;
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
