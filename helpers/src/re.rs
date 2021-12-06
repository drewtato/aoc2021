use regex::Regex;

use crate::{MultiFromStr, MultiParse, MultiParseError};

/// Takes a regex and parses the capture groups into a container.
///
/// # Examples
/// ```
/// # use regex::Regex;
/// # use helpers::re_parse;
/// let reg = Regex::new(r"(\d+)=(\d+)").unwrap();
/// let text = "10=5,2=2,321=76,43425=643,234=3427";
///
/// let arr: [usize; 10] = re_parse(&reg, text).unwrap();
///
/// assert_eq!(arr, [10, 5, 2, 2, 321, 76, 43425, 643, 234, 3427])
/// ```
pub fn re_parse<T>(reg: &Regex, text: &str) -> Result<T, MultiParseError>
where
	T: MultiFromStr,
{
	reg.captures_iter(text)
		.flat_map(|caps| {
			caps.iter()
				.skip(1)
				.flat_map(|cap| cap.map(|m| m.as_str().to_string()))
				.collect::<Vec<String>>()
		})
		.multi_parse()
}
