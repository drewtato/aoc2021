#![feature(array_from_fn)]
#![feature(hash_drain_filter)]

use std::{
	fmt::{Debug, Display},
	io::{stdin, stdout, BufRead, Read, Write},
};

pub use itertools;
pub use regex;

pub type BoxErr = Box<dyn std::error::Error>;

pub fn read_stdin() -> Result<String, BoxErr> {
	let mut buf = String::new();
	stdin().lock().read_to_string(&mut buf)?;
	Ok(buf)
}

/// Prompts stdin for input and returns trimmed input.
pub fn input() -> String {
	print!("> ");
	stdout().flush().unwrap();
	let mut buf = String::new();
	stdin().lock().read_line(&mut buf).unwrap();
	buf.trim().to_string()
}

pub fn display<T: Display>(value: T) {
	println!("{}", value);
}

pub fn debug<T: Debug>(value: T) {
	println!("{:?}", value);
}

mod multi_parse;
pub use multi_parse::{MultiFromStr, MultiParse, MultiParseError};
// mod bi_map;
// pub use bi_map::BiMap;
