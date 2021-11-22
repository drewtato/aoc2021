use std::fs::read_dir;

use pico_args::Arguments;
use regex::Regex;
use reqwest::Url;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const YEAR: u32 = 2021;

fn main() -> Result<()> {
	let mut args = Arguments::from_env();

	if let Some(sub) = args.subcommand()? {
		let day: u32 = if let Some(day) = args.opt_value_from_str("-d")? {
			day
		} else {
			latest_day()?
		};

		let year: u32 = args.opt_value_from_str("-y")?.unwrap_or(YEAR);
		match sub.as_str() {
			"fetch" => fetch(day, year)?,
			"run" => run(day, year)?,
			_ => return Err("Unknown subcommand".into()),
		}
	} else {
		eprintln!(
			"Usage: runner <subcommand> [-d <number>] [-y <number>]
Year and day are optional (default to year {YEAR} and the most recent day in repo)
  `fetch`: download input
  `run`: run a specific day"
		);
	}
	Ok(())
}

fn run(day: u32, year: u32) -> Result<()> {
	todo!()
}

fn fetch(day: u32, year: u32) -> Result<()> {
	todo!()
}

fn latest_day() -> Result<u32> {
	let re: Regex = r"day(\d{2})".parse().unwrap();

	read_dir("./days")
		.or_else(|_| read_dir("../days"))?
		.filter_map(|entry| entry.ok())
		.filter_map(|entry| entry.file_type().ok().map(|ftype| (ftype.is_dir(), entry)))
		.filter_map(|(is_dir, entry)| {
			if is_dir {
				re.captures(&entry.file_name().to_string_lossy())
					.and_then(|caps| caps.get(1))
					.and_then(|m| m.as_str().parse().ok())
			} else {
				None
			}
		})
		.max()
		.or(Some(1))
		.ok_or_else(|| unreachable!())
}

fn input_url(year: u32, day: u32) -> Result<Url> {
	format!("https://adventofcode.com/{year}/day/{day}/input")
		.parse()
		.map_err(Into::into)
}
