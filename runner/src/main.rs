// #![allow(unused, dead_code)]

use fs_extra::dir::CopyOptions;
use pico_args::Arguments;
use std::{
	fs::{create_dir_all, read_to_string, File, OpenOptions},
	io::Write,
	path::Path,
	process::{Command, Stdio},
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

	if args.contains("-h") {
		println!("{}", HELP);
		return Ok(());
	}

	let project_root = project_root().ok_or("Could not find project root")?;

	let sub = args.subcommand()?;

	let runner_options = RunnerOptions {
		only_setup: args.contains("-s"),
		year: args.opt_value_from_str("-y")?.unwrap_or(YEAR),
		project_root: &project_root,
		get_input: !args.contains("-i"),
		test: args.contains("-t"),
		release: args.contains("-r"),
		execute_only: args.contains("-e"),
	};

	if let Some(s) = sub {
		if &s == "all" {
			(1..=latest_day(&project_root)?).try_for_each(|day| runner_options.runner(day))
		} else {
			let day = s.parse().map_err(|_| {
				eprintln!("{}", HELP);
				"Could not parse day as number"
			})?;

			if day > 25 {
				return Err(format!("Day can't be greater than 25\nDay: {}", day).into());
			}

			runner_options.runner(day)
		}
	} else {
		let day = latest_day(&project_root)?;

		runner_options.runner(day)
	}
}

#[derive(Debug, Clone, Copy)]
struct RunnerOptions<'a> {
	only_setup: bool,
	year: u32,
	project_root: &'a Path,
	get_input: bool,
	test: bool,
	release: bool,
	execute_only: bool,
}

impl RunnerOptions<'_> {
	fn runner(self, day: u32) -> Result<()> {
		if self.execute_only {
			self.run_from_target(day)?;
		} else {
			self.setup(day)?;
			if !self.only_setup {
				self.run_with_input(day)?;
			}
		}
		Ok(())
	}

	fn run_from_target(self, day: u32) -> Result<()> {
		let program = if cfg!(windows) {
			format!(
				"target/{}/day{day:02}.exe",
				if self.release { "release" } else { "test" }
			)
		} else {
			format!(
				"target/{}/day{day:02}",
				if self.release { "release" } else { "test" }
			)
		};
		let mut cmd = Command::new(program)
			.current_dir(self.project_root)
			.stdin(Stdio::piped())
			.spawn()?;

		std::io::copy(&mut self.input_file(day)?, &mut cmd.stdin.take().unwrap())?;

		cmd.wait()?;

		Ok(())
	}

	fn run_with_input(self, day: u32) -> Result<()> {
		eprintln!(
			"Running day {day} in {} with {} input",
			if self.release { "release" } else { "debug" },
			if self.test { "test" } else { "real" }
		);
		cargo_in(
			["run", "--package", &format!("day{day:02}")]
				.into_iter()
				.chain(self.release.then(|| "--release")),
			self.project_root,
			self.input_file(day)?,
		)
	}

	fn input_file(self, day: u32) -> Result<File> {
		let input_file = File::open(self.project_root.join(format!(
			"inputs/{}{day:02}.txt",
			if self.test { "test" } else { "day" }
		)))?;
		Ok(input_file)
	}

	fn setup(self, day: u32) -> Result<()> {
		if self.get_input {
			let input_path = self.project_root.join(format!("inputs/day{day:02}.txt"));
			if !input_path.exists() {
				eprintln!("Getting input");
				let url = input_url(day, self.year)?;
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
							"Error from AOC server: {e} {}",
							if e.status().unwrap().as_u16() == 404 {
								"It is likely the puzzle is not released yet"
							} else {
								""
							}
						)
					})?
					.text()?;

				write_to_file(&request, input_path).map_err(|_| "Problem writing input to file")?;
			}

			let test_path = self.project_root.join(format!("inputs/test{day:02}.txt"));
			if !test_path.exists() {
				write_to_file("", test_path).map_err(|_| "Problem writing test to file")?;
			}
		} else {
			eprintln!("Skipping input");
		}

		let member_path = self.project_root.join(format!("days/day{day:02}"));

		if !member_path.is_dir() {
			eprintln!("Setting up package");
			create_dir_all(&member_path)?;

			cargo(["init", "--vcs", "none"], &member_path)
				.map_err(|_| "Cargo failed to create new project")?;

			fs_extra::dir::copy(
				&self.project_root.join("template/src"),
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
				read_to_string(self.project_root.join("template/Cargo.append.toml"))?
			)?;
		}

		Ok(())
	}
}
