// #![allow(unused, dead_code)]

use fs_extra::dir::CopyOptions;
use pico_args::Arguments;
use std::{
	fs::{create_dir_all, read_to_string, OpenOptions},
	io::Write,
	path::Path,
};

mod utils;
use reqwest::{blocking::Client, header::COOKIE};
use utils::*;

type BoxErr = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, BoxErr>;
const YEAR: u32 = 2021;

fn main() -> Result<()> {
	dotenv::dotenv()?;
	let mut args = Arguments::from_env();

	let project_root =
		project_root().ok_or_else::<BoxErr, _>(|| "Could not find project root".into())?;

	if let Some(sub) = args.subcommand()? {
		let day: u32 = if let Some(day) = args.opt_value_from_str("-d")? {
			day
		} else {
			latest_day(&project_root)?
		};

		let year: u32 = args.opt_value_from_str("-y")?.unwrap_or(YEAR);
		let input: bool = args.contains("-i");
		match sub.as_str() {
			"fetch" => fetch(day, year, &project_root, !input)?,
			"run" => run(day, &project_root)?,
			"test" => test_inp(day, &project_root)?,
			_ => return Err("Unknown subcommand".into()),
		}
	} else {
		eprintln!(
			"Usage: runner <subcommand> [-d <number>] [-y <number>] [-i]

Year and day are optional (default to year {YEAR} and the most recent day in repo). The year
is most useful for testing on previous years in preparation for December.

  -d    - Day
  -y    - Year
  -i    - Do not retrieve input

Subcommands:
  fetch - download input
  run   - run a specific day"
		);
	}
	Ok(())
}

fn test_inp(day: u32, project_root: &Path) -> Result<()> {
	todo!("{day}, {:?}", project_root)
}

fn run(day: u32, project_root: &Path) -> Result<()> {
	todo!("{day}, {:?}", project_root)
}

fn fetch(day: u32, year: u32, project_root: &Path, get_input: bool) -> Result<()> {
	if get_input {
		let url = input_url(day, year)?;
		let client = Client::new();

		let request = client
			.get(url)
			.header(
				COOKIE,
				format!(
					"session={}",
					dotenv::var("SESSION").map_err(|_| "No session cookie available")?
				),
			)
			.send()?
			.error_for_status()
			.map_err(|e| {
				format!(
					"Error from AOC server: {e}{}",
					if e.status().unwrap().as_u16() == 404 {
						"\nIt is likely the puzzle is not released yet"
					} else {
						""
					}
				)
			})?
			.text()?;

		write_to_file(
			&request,
			project_root.join(format!("inputs/day{day:02}.txt")),
		)
		.map_err(|_| "Problem writing input to file")?;
	} else {
		println!("Skipping input");
	}

	let member_path = project_root.join(format!("days/day{day:02}"));

	if !member_path.is_dir() {
		create_dir_all(&member_path)?;

		cargo(["init", "--vcs", "none"], &member_path)
			.map_err(|_| "Cargo failed to create new project")?;

		fs_extra::dir::copy(
			&project_root.join("template/src"),
			&member_path,
			&CopyOptions {
				overwrite: true,
				..Default::default()
			},
		)?;

		let cargo_toml = member_path.join("Cargo.toml");
		let mut f = OpenOptions::new().append(true).open(&cargo_toml)?;
		write!(
			f,
			"{}",
			read_to_string(project_root.join("template/Cargo.append.toml"))?
		)?;
	}

	Ok(())
}
