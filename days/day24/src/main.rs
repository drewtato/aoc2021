use helpers::itertools::Itertools;
use helpers::*;

type Input = Vec<(u8, u8, Result<Data, u8>)>;
type Data = i64;

const INP: u8 = 1;
const MUL: u8 = 2;
const ADD: u8 = 3;
const DIV: u8 = 4;
const EQL: u8 = 5;
const MOD: u8 = 6;

// const W: u8 = 1;
// const X: u8 = 2;
// const Y: u8 = 3;
// const Z: u8 = 4;

fn parser() -> Input {
	read_stdin()
		.unwrap()
		.trim()
		.lines()
		.map(|l| l.trim())
		.map(|l| {
			(
				match &l[0..3] {
					"inp" => INP,
					"mul" => MUL,
					"add" => ADD,
					"div" => DIV,
					"eql" => EQL,
					"mod" => MOD,
					_ => unreachable!(),
				},
				(l.chars().nth(4).unwrap() as u8) - b'v',
				if l.len() > 5 {
					l[6..]
						.parse()
						.map_err(|_| (l.chars().nth(6).unwrap() as u8) - b'v')
				} else {
					Err(0)
				},
			)
		})
		.collect()
}

fn main() {
	let inp = parser();
	// for line in inp {
	// 	debug(line);
	// }

	let div = inp
		.citer()
		.skip(4)
		.step_by(18)
		.map(|(_, _, n)| n.unwrap() == 26);
	let offset1 = inp.citer().skip(5).step_by(18).map(|(_, _, n)| n.unwrap());
	let offset2 = inp.citer().skip(15).step_by(18).map(|(_, _, n)| n.unwrap());

	let all_changes = div
		.zip(offset1)
		.zip(offset2)
		.map(|((div, o1), o2)| (div, o1, o2))
		.collect_vec();

	// let combined = all_changes
	// 	.array_windows()
	// 	.map(|&[(_, _, o2, _), (_, o1, _, _)]| o1 + o2)
	// 	.collect_vec();
	// // debug(&combined);

	let mut stack = Vec::with_capacity(7);
	let mut pairs = Vec::with_capacity(7);

	for (i, &(div, of1, of2)) in all_changes.iter().enumerate() {
		if !div {
			stack.push((i, of2));
		} else {
			let prev = stack.pop().unwrap();
			let diff = of1 + prev.1;
			pairs.push((prev.0, i, diff));
		}
	}

	// debug(pairs);
	let mut max_digits = [0; 14];
	let mut min_digits = [0; 14];

	for (a, b, diff) in pairs {
		if diff > 0 {
			min_digits[a] = 1;
			min_digits[b] = 1 + diff;

			max_digits[a] = 9 - diff;
			max_digits[b] = 9;
		} else {
			min_digits[a] = 1 - diff;
			min_digits[b] = 1;

			max_digits[a] = 9;
			max_digits[b] = 9 + diff;
		}
	}

	#[cfg(debug_assertions)]
	{
		assert_eq!(0, run_monad(&inp, max_digits.citer())[3]);
		assert_eq!(0, run_monad(&inp, min_digits.citer())[3]);
	}

	display(max_digits.iter().join(""));
	display(min_digits.iter().join(""));

	// //                    v                                      v
	// //                       v                                v
	// //                          v                          v
	// //                             v              v
	// //                                v  v
	// //                                      v  v
	// //                                               v  v
	// let saved = [2, 1, 1, 9, 1, 8, 6, 1, 1, 5, 1, 1, 6, 1];
	// let mut z = 0;
	// for (i, (digit, &(div, o1, o2, max))) in digits.iter_mut().zip(&all_changes).enumerate() {
	// 	let com = combined.get(i).copied().unwrap_or(0);
	// 	println!(
	// 		"{i} Div: {}, o1: {o1}, o2: {o2}, max: {max}",
	// 		if div { "yes" } else { "no" }
	// 	);
	// 	let off = z % 26 + o1;
	// 	println!("z % 26 + offset: {}, combined: {}", off, com);
	// 	*digit = if (1..=9).contains(&off) {
	// 		off
	// 	} else {
	// 		saved.get(i).copied().unwrap_or_else(input)
	// 	};

	// 	println!("Digit: {digit}");
	// 	sim(*digit, &mut z, div, o1, o2);
	// 	let bases = base_26(z);
	// 	display(bases.iter().join(":"));
	// }

	// let z = run_monad(&inp, digits.citer())[4];
	// assert_eq!(z, 0);
	// display(digits.iter().join(""));
}

// fn base_26(mut z: Data) -> Vec<Data> {
// 	if z == 0 {
// 		return vec![0];
// 	}
// 	let mut v = Vec::new();
// 	while z > 0 {
// 		v.push(z % 26);
// 		z /= 26;
// 	}
// 	v
// }

// fn dec(digits: &mut [Data]) -> bool {
// 	let last = digits.last_mut().unwrap();
// 	*last -= 1;
// 	if *last == 0 {
// 		*last = 9;
// 		let len = digits.len();
// 		if len > 1 {
// 			dec(&mut digits[..(len - 1)])
// 		} else {
// 			false
// 		}
// 	} else {
// 		true
// 	}
// }

// fn sim(digit: Data, z: &mut Data, div: bool, offset1: Data, offset2: Data) {
// 	let condition = digit == (*z % 26) + offset1;
// 	if div {
// 		*z /= 26;
// 	}
// 	if !condition {
// 		*z *= 26;
// 		*z += digit + offset2;
// 	}
// }

#[cfg(debug_assertions)]
fn run_monad<I>(program: &[(u8, u8, Result<Data, u8>)], mut inputs: I) -> [Data; 4]
where
	I: Iterator<Item = Data>,
{
	let mut registers = [0; 4];
	for &(ins, reg, second) in program {
		// debug(registers);
		let reg = reg as usize - 1;
		match ins {
			INP => {
				registers[reg] = inputs.next().unwrap();
			}
			ADD => {
				registers[reg] = registers[reg]
					.checked_add(second.unwrap_or_else(|reg| registers[reg as usize - 1]))
					.unwrap();
			}
			MUL => {
				registers[reg] = registers[reg]
					.checked_mul(second.unwrap_or_else(|reg| registers[reg as usize - 1]))
					.unwrap();
			}
			DIV => {
				registers[reg] = registers[reg]
					.checked_div(second.unwrap_or_else(|reg| registers[reg as usize - 1]))
					.unwrap();
			}
			MOD => {
				registers[reg] %= second.unwrap_or_else(|reg| registers[reg as usize - 1]);
			}
			EQL => {
				registers[reg] = (registers[reg]
					== second.unwrap_or_else(|reg| registers[reg as usize - 1]))
					as Data;
			}
			_ => unreachable!(),
		}
	}
	registers
}
