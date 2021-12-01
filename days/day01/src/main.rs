use helpers::{display, read_stdin};

type Input = Vec<u32>;

fn parser() -> Input {
	read_stdin()
		.unwrap()
		.trim()
		.lines()
		.map(|l| l.parse().unwrap())
		.collect()
}

fn main() {
	let inp = parser();

	// Part 1
	let counter = inp.windows(2).filter(|&i| i[0] < i[1]).count();
	display(counter);

	// Part 2
	let counter = inp.windows(4).filter(|&i| i[0] < i[3]).count();
	display(counter);
}
