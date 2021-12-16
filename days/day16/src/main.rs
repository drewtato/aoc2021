#![allow(unused_imports)]
use helpers::itertools::Itertools;
use helpers::*;

type Input = Vec<u8>;

fn parser() -> Input {
	read_stdin()
		.unwrap()
		.trim()
		.chars()
		.flat_map(|c| {
			let mut digit = c.to_digit(16).unwrap() as u8;
			let a = digit & 1;
			digit >>= 1;
			let b = digit & 1;
			digit >>= 1;
			let c = digit & 1;
			digit >>= 1;
			let d = digit;
			[d, c, b, a]
		})
		.collect()
}

fn main() {
	let inp = parser();
	// display_bits(&inp);

	// debug(&inp);

	let mut inp_iter = inp.citer();
	let mut message = Vec::new();
	while let Some((a, b, c, d, e, f)) = inp_iter.next_tuple() {
		let version = (a << 2) + (b << 1) + c;
		let type_id = (d << 2) + (e << 1) + f;
		if version == 0 && type_id == 0 {
			break;
		}
		let mut read = 6;
		let packet = match type_id {
			4 => {
				let mut value: u64 = 0;
				loop {
					let flag = inp_iter.next().unwrap();
					value <<= 4;
					value += get_number(&mut inp_iter, 4);
					read += 5;
					if flag == 0 {
						break;
					}
				}
				Packet {
					packet_type: Literal(value),
					version,
					length: read,
				}
			}
			t => {
				let length_type = inp_iter.next().unwrap();
				read += 1;
				let op_type = match t {
					0 => Sum,
					1 => Product,
					2 => Min,
					3 => Max,
					5 => Greater,
					6 => Less,
					7 => Equal,
					_ => unreachable!(),
				};
				if length_type == 0 {
					Packet {
						packet_type: OperatorLength(
							op_type,
							get_number(&mut inp_iter, 15) as usize,
						),
						version,
						length: read + 15,
					}
				} else {
					Packet {
						packet_type: OperatorSubPackets(
							op_type,
							get_number(&mut inp_iter, 11) as usize,
						),
						version,
						length: read + 11,
					}
				}
			}
		};
		// debug(packet);
		message.push(packet);
	}
	let version_sum: u64 = message.citer().map(|packet| packet.version as u64).sum();
	display(version_sum);

	// dbg!(&message);

	let results = compute_message(&mut message.into_iter()).0;
	display(results);
}

fn compute_message<I: Iterator<Item = Packet>>(message: &mut I) -> (u64, usize) {
	let outer = message.next().unwrap();

	let mut stack = Vec::new();
	let op_type = match outer.packet_type {
		Literal(v) => return (v, outer.length),
		OperatorLength(op_type, mut lens) => {
			let mut inner = Vec::new();
			while lens > 0 {
				let pack = message.next().unwrap();
				lens -= pack.length;
				inner.push(pack);
			}
			let mut inner = inner.into_iter();
			while inner.len() > 0 {
				stack.push(compute_message(&mut inner));
			}
			op_type
		}
		OperatorSubPackets(op_type, subs) => {
			for _ in 0..subs {
				stack.push(compute_message(message));
			}
			op_type
		}
	};
	calculate(stack, op_type)
}

fn calculate<I: IntoIterator<Item = (u64, usize)>>(
	stack: I,
	op_type: OperatorType,
) -> (u64, usize) {
	let (stack_vec, count_vec): (Vec<u64>, Vec<usize>) = stack.into_iter().unzip();
	let count = count_vec.into_iter().sum();
	let mut stack_iter = stack_vec.into_iter();
	(
		match op_type {
			Sum => stack_iter.sum(),
			Product => stack_iter.product(),
			Min => stack_iter.min().unwrap(),
			Max => stack_iter.max().unwrap(),
			Greater => (stack_iter.next().unwrap() > stack_iter.next().unwrap()) as u64,
			Less => (stack_iter.next().unwrap() < stack_iter.next().unwrap()) as u64,
			Equal => (stack_iter.next().unwrap() == stack_iter.next().unwrap()) as u64,
		},
		count,
	)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Packet {
	packet_type: PacketType,
	version: u8,
	length: usize,
}

use PacketType::*;
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum PacketType {
	Literal(u64),
	OperatorLength(OperatorType, usize),
	OperatorSubPackets(OperatorType, usize),
}
use OperatorType::*;
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum OperatorType {
	Sum,
	Product,
	Min,
	Max,
	Greater,
	Less,
	Equal,
}

// fn display_bits(inp: &[u8]) {
// 	for n in inp {
// 		print!("{n:08b}");
// 	}
// 	println!();
// }

fn get_number<I: Iterator<Item = u8>>(inp: &mut I, count: usize) -> u64 {
	let mut value = 0;
	for _ in 0..count {
		let bit = inp.next().unwrap();
		value <<= 1;
		value += bit as u64;
	}
	value
}
