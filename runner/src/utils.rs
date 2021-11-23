use std::{
	fs::{create_dir_all, read_dir, File},
	io::Write,
	path::{Path, PathBuf},
	process::Command,
};

use crate::Result;
use regex::Regex;
use reqwest::Url;

pub fn project_root() -> Option<PathBuf> {
	Path::new("./Cargo.lock")
		.is_file()
		.then(|| ".")
		.or_else(|| Path::new("../Cargo.lock").is_file().then(|| ".."))
		.map(|s| s.into())
}

pub fn latest_day(project_root: &Path) -> Result<u32> {
	let re: Regex = r"^day(\d+)$".parse().unwrap();

	let project_root = project_root.join("days");

	read_dir(project_root)?
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

pub fn input_url(day: u32, year: u32) -> Result<Url> {
	format!("https://adventofcode.com/{year}/day/{day}/input")
		.parse()
		.map_err(Into::into)
}

pub fn write_to_file<P: AsRef<Path>>(s: &str, path: P) -> Result<()> {
	create_dir_all(path.as_ref().parent().unwrap())?;
	let mut f = File::create(path.as_ref())?;
	write!(f, "{s}")?;
	Ok(())
}

pub fn cargo<I, S, P>(args: I, dir: P) -> Result<()>
where
	I: IntoIterator<Item = S>,
	S: AsRef<std::ffi::OsStr>,
	P: AsRef<Path>,
{
	let status = Command::new("cargo")
		.current_dir(dir)
		.args(args)
		.spawn()?
		.wait()?;
	if !status.success() {
		Err("Cargo failed".into())
	} else {
		Ok(())
	}
}
