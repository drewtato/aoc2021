use helpers::read_stdin;

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
	println!("{:?}", parser());
}
