use std::{collections::HashMap, fmt::Debug, hash::Hash};

/// A [`HashMap`]-backed collection that has an element at every key.
///
/// All values that are physically
/// present in the `HashMap` are non-default, except when the map has been edited by direct access
/// or [`entry`](DefaultMap::entry). [`clean`](DefaultMap::clean) fixes this in almost all cases.
///
/// Two ways for default values to be in the map are when the [`Default`] and [`PartialEq`] impls
/// are done such that `default() != default()`, and from interior mutability (like
/// [`RefCell`](std::cell::RefCell)).
#[derive(Default, Clone)]
pub struct DefaultMap<K, V>(pub HashMap<K, V>);

impl<K: Debug, V: Debug> Debug for DefaultMap<K, V> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		Debug::fmt(&self.0, f)
	}
}

impl<K, V> DefaultMap<K, V> {
	pub fn new() -> Self {
		DefaultMap(HashMap::new())
	}

	pub fn with_map(hash_map: HashMap<K, V>) -> Self {
		DefaultMap(hash_map)
	}
}

impl<K: Eq + Hash, V: Default + PartialEq> DefaultMap<K, V> {
	/// Gets rid of any default values in the map.
	pub fn clean(&mut self) {
		let default = Default::default();
		self.0.retain(|_, v| v != &default);
	}
}

impl<K: Eq + Hash, V: Default> DefaultMap<K, V> {
	pub fn entry(&mut self, key: K) -> &mut V {
		self.0.entry(key).or_default()
	}
}
impl<K: Eq + Hash, V: Default + PartialEq> DefaultMap<K, V> {
	/// Sets a key-value pair if it isn't default. Returns the value at that location if one existed.
	pub fn insert(&mut self, key: K, value: V) -> Option<V> {
		if value != Default::default() {
			self.0.insert(key, value)
		} else {
			None
		}
	}
}

impl<K: Eq + Hash, V: Default + Clone> DefaultMap<K, V> {
	pub fn get(&self, key: &K) -> V {
		self.0.get(key).cloned().unwrap_or_default()
	}
}
