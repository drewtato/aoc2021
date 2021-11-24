// #![allow(unused, dead_code)]

use fs_extra::dir::CopyOptions;
use pico_args::Arguments;
use std::{
	fs::{create_dir_all, read_to_string, File, OpenOptions},
	io::Write,
	path::Path,
};

use reqwest::{blocking::Client, header::COOKIE};

mod utils;
use utils::*;

type BoxErr = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, BoxErr>;

const YEAR: u32 = 2021;
const HELP: &str = include_str!("../help.txt");

fn main() -> Result<()> {
	dotenv::dotenv()?;
	let mut args = Arguments::from_env();

	let project_root =
		project_root().ok_or_else::<BoxErr, _>(|| "Could not find project root".into())?;

	let sub = args.subcommand()?;

	let day: u32 = if let Some(s) = sub {
		s.parse().map_err(|_| {
			eprintln!("{}", HELP);
			"Could not parse day as number"
		})?
	} else {
		latest_day(&project_root)?
	};

	if args.contains("-h") {
		println!("{}", HELP);
		return Ok(());
	}

	let get_input = !args.contains("-i");
	let release = args.contains("-r");
	let year = args.opt_value_from_str("-y")?.unwrap_or(YEAR);

	setup(day, year, &project_root, get_input)?;
	if args.contains("-s") {
		return Ok(());
	}

	if args.contains("-t") {
		test_inp(day, &project_root, release)
	} else {
		run(day, &project_root, release)
	}?;

	Ok(())
}

fn run(day: u32, project_root: &Path, release: bool) -> Result<()> {
	eprintln!(
		"Running day {day} in {} with real input",
		if release { "release" } else { "debug" }
	);
	let input_file = File::open(project_root.join(format!("inputs/day{day:02}.txt")))?;
	run_with_input(day, project_root, input_file, release)
}

fn test_inp(day: u32, project_root: &Path, release: bool) -> Result<()> {
	eprintln!(
		"Running day {day} in {} with test input",
		if release { "release" } else { "debug" }
	);
	let input_file = File::open(project_root.join(format!("inputs/test{day:02}.txt")))?;
	run_with_input(day, project_root, input_file, release)
}

fn run_with_input(day: u32, project_root: &Path, input_file: File, release: bool) -> Result<()> {
	cargo_in(
		["run", "--package", &format!("day{day:02}")]
			.into_iter()
			.chain(release.then(|| "--release")),
		project_root,
		input_file,
	)
}

fn setup(day: u32, year: u32, project_root: &Path, get_input: bool) -> Result<()> {
	if get_input {
		let input_path = project_root.join(format!("inputs/day{day:02}.txt"));
		if !input_path.exists() {
			eprintln!("Getting input");
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

			write_to_file(&request, input_path).map_err(|_| "Problem writing input to file")?;
		}

		let test_path = project_root.join(format!("inputs/test{day:02}.txt"));
		if !test_path.exists() {
			write_to_file("", test_path).map_err(|_| "Problem writing test to file")?;
		}
	} else {
		eprintln!("Skipping input");
	}

	let member_path = project_root.join(format!("days/day{day:02}"));

	if !member_path.is_dir() {
		eprintln!("Setting up package");
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
