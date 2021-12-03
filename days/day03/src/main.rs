use helpers::{display, read_stdin, VecParse};

type Input = Vec<Vec<usize>>;

fn parser() -> Input {
	read_stdin()
		.unwrap()
		.trim()
		.lines()
		.map(|l| l.split_inclusive(|_| true).vec_parse().unwrap())
		.collect()
}

fn main() {
	let inp = parser();
	let size = inp[0].len();

	// Part 1
	let gamma_rate = bits_to_num((0..size).map(|index| {
		(inp.iter().filter(|&bits| bits[index] == 1).count() > inp.len() / 2) as usize
	}));
	// display(gamma_rate);
	let epsilon_rate = gamma_rate ^ bits_to_num(vec![1; size]);
	// display(epsilon_rate);
	display(gamma_rate * epsilon_rate);

	// Part 2
	let mut common = inp.clone();
	let mut index = 0;
	let oxygen_generator_rating = loop {
		let [most, _] = partition_common(&common, index);
		common = most;
		if common.len() == 1 {
			break bits_to_num(common[0].clone());
		}
		index += 1;
	};
	let mut common = inp;
	let mut index = 0;
	let co2_scrubber_rating = loop {
		let [_, least] = partition_common(&common, index);
		common = least;
		if common.len() == 1 {
			break bits_to_num(common[0].clone());
		}
		index += 1;
	};
	// display(oxygen_generator_rating);
	// display(co2_scrubber_rating);
	display(oxygen_generator_rating * co2_scrubber_rating);
}

fn partition_common(arr: &[Vec<usize>], pos: usize) -> [Vec<Vec<usize>>; 2] {
	let (ones, zeroes) = arr
		.iter()
		.cloned()
		.partition::<Vec<_>, _>(|bits| bits[pos] == 1);
	if ones.len() >= zeroes.len() {
		[ones, zeroes]
	} else {
		[zeroes, ones]
	}
}

fn bits_to_num<I: IntoIterator<Item = usize>>(bits: I) -> usize {
	bits.into_iter()
		.reduce(|acc: usize, bit| (acc << 1) + bit)
		.unwrap()
}
