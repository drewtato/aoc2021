#![allow(dead_code)]
use std::collections::HashMap;
use std::hash::Hash;

// use crate::SignedVec;

pub trait CellularAutomaton {
	fn automate(&mut self);
}

#[derive(Debug, Default, Clone)]
pub struct HashMapCA<K, V, R, T, F> {
	map: HashMap<K, V>,
	temp_map: HashMap<K, T>,
	rule: R,
	finalize: F,
}

impl<K: Eq + Hash, V, R, T, F> HashMapCA<K, V, R, T, F> {
	pub fn new(rule: R, finalize: F) -> Self {
		Self::with_map(HashMap::new(), rule, finalize)
	}
	pub fn with_map(map: HashMap<K, V>, rule: R, finalize: F) -> Self {
		Self {
			map,
			temp_map: HashMap::new(),
			rule,
			finalize,
		}
	}
}

impl<K, V, R, T, F> CellularAutomaton for HashMapCA<K, V, R, T, F> {
	fn automate(&mut self) {
		todo!()
	}
}

// #[derive(Debug, Default, Clone)]
// pub struct SignedCA<G, R> {
// 	pub grid: G,
// 	alt_grid: G,
// 	rule: R,
// }

// impl<T: Clone + Default, R> SignedCA<SignedVec<SignedVec<T>>, R> {
// 	pub fn new<I: IntoIterator<Item = ([isize; 2], T)>>(grid: I, rule: R) -> Self {
// 		let grid_iter = grid.into_iter();
// 		let mut grid = SignedVec::new(0);
// 		for ([y, x], value) in grid_iter {
// 			if grid.len() as isize >= y {
// 				let mut new_row = SignedVec::new(x);
// 				new_row.set(x, value);
// 				grid.set(y, new_row);
// 			} else {
// 				grid[y].set(x, value);
// 			}
// 		}

// 		Self {
// 			grid: grid.clone(),
// 			alt_grid: grid,
// 			rule,
// 		}
// 	}
// }
