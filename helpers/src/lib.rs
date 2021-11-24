use std::io::Read;

type BoxErr = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, BoxErr>;

pub fn read_stdin() -> Result<String> {
	let mut buf = String::new();
	std::io::stdin().lock().read_to_string(&mut buf)?;
	Ok(buf)
}
