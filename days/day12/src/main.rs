use std::collections::HashMap;

use helpers::*;

type Input = Vec<(String, String)>;

fn parser() -> Input {
	let re = regex::Regex::new(r"([A-Za-z]+)").unwrap();
	read_stdin()
		.unwrap()
		.trim()
		.lines()
		.map(|l| re_parse(&re, l).unwrap())
		.collect()
}

type Key = i8;
const START: Key = 0;
const END: Key = 1;

fn main() {
	let inp = parser();

	let mut strings = HashMap::new();
	strings.insert("start", START);
	strings.insert("end", END);
	let map: HashMap<Key, Vec<Key>> =
		inp.iter()
			.fold(HashMap::with_capacity(inp.len()), |mut map, (a, b)| {
				let next = strings.len() as Key;
				let a = *strings.entry(a.as_str()).or_insert_with(|| {
					if a.chars().next().unwrap().is_lowercase() {
						-next
					} else {
						next
					}
				});
				let next = strings.len() as Key;
				let b = *strings.entry(b.as_str()).or_insert_with(|| {
					if b.chars().next().unwrap().is_lowercase() {
						-next
					} else {
						next
					}
				});
				if a != START && b != END {
					map.entry(b).or_default().push(a);
				}
				if b != START && a != END {
					map.entry(a).or_default().push(b);
				}
				map
			});

	let finished_paths = num_paths::<false>(&map);
	display(finished_paths);

	let finished_paths = num_paths::<true>(&map);
	display(finished_paths);
}

fn num_paths<const ALLOW_SMALL_TWICE: bool>(map: &HashMap<Key, Vec<Key>>) -> usize {
	let mut path = vec![(START, 0u8)];
	let mut visited = [0u8; 256];
	let mut small_visited_twice = false;
	let mut total = 0;

	visited[i8_to_usize(START)] += 1;

	while let Some(&mut (node, ref mut index_mut)) = path.last_mut() {
		let index = *index_mut;
		*index_mut += 1;
		if let Some(&next) = map[&node].get(index as usize) {
			if next < 0 && visited[i8_to_usize(next)] > 0 {
				if ALLOW_SMALL_TWICE && !small_visited_twice {
					small_visited_twice = true;
				} else {
					continue;
				}
			} else if next == END {
				total += 1;
				continue;
			}
			path.push((next, 0));
			visited[i8_to_usize(next)] += 1;
		} else {
			let (node, _) = path.pop().unwrap();
			let vis = &mut visited[i8_to_usize(node)];
			if ALLOW_SMALL_TWICE && node < 0 && *vis == 2 {
				small_visited_twice = false;
			}
			*vis -= 1;
		}
	}
	total
}

fn i8_to_usize(i: i8) -> usize {
	((i as i16) + 128) as _
}
