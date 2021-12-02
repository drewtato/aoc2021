use std::{
	fmt::{Debug, Display},
	io::Read,
};

pub use itertools;
pub use reformation;
pub use regex;

pub type BoxErr = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, BoxErr>;

pub fn read_stdin() -> Result<String> {
	let mut buf = String::new();
	std::io::stdin().lock().read_to_string(&mut buf)?;
	Ok(buf)
}

pub fn display<T: Display>(value: T) {
	println!("{}", value);
}

pub fn debug<T: Debug>(value: T) {
	println!("{:?}", value);
}
