use std::{collections::HashMap, hash::Hash};

/// A bidirectional hash table. Each `left` element is associated with any number of `right`
/// elements, and each `right` element is associated with any number of `left` elements.
#[derive(Debug, Default, Clone)]
pub struct BiMap<L, R> {
	left_elements: Vec<L>,
	right_elements: Vec<R>,
	left_map: HashMap<L, Vec<usize>>,
	right_map: HashMap<R, Vec<usize>>,
}

impl<L, R> BiMap<L, R> {
	/// Creates a new, empty BiMap
	pub fn new() -> Self {
		BiMap {
			left_elements: Vec::new(),
			right_elements: Vec::new(),
			left_map: HashMap::new(),
			right_map: HashMap::new(),
		}
	}

	/// Creates a new, empty BiMap with specified capacities.
	pub fn with_capacities(left: usize, right: usize) -> Self {
		BiMap {
			left_elements: Vec::with_capacity(left),
			right_elements: Vec::with_capacity(right),
			left_map: HashMap::with_capacity(left),
			right_map: HashMap::with_capacity(right),
		}
	}
}

impl<L: Hash + Eq + Clone, R: Hash + Eq + Clone> BiMap<L, R> {
	pub fn from_map(map: HashMap<L, Vec<R>>) -> Self {
		let r_cap = map.values().map(|v| v.len()).sum();
		let mut left_elements = Vec::with_capacity(map.len());
		let mut right_elements = Vec::with_capacity(r_cap);
		let mut left_map: HashMap<L, Vec<usize>> = HashMap::with_capacity(map.len());
		let mut right_map: HashMap<R, Vec<usize>> = HashMap::with_capacity(r_cap);

		for (k, vs) in map {
			left_elements.push(k.clone());
			for v in vs {
				right_elements.push(v.clone());
				left_map
					.entry(k.clone())
					.or_default()
					.push(left_elements.len() - 1);
				right_map
					.entry(v)
					.or_default()
					.push(right_elements.len() - 1);
			}
		}

		BiMap {
			left_elements,
			right_elements,
			left_map,
			right_map,
		}
	}
}
