use std::{
	collections::{
		hash_map::{self, Keys, Values},
		hash_set, vec_deque, BinaryHeap, HashMap, HashSet, VecDeque,
	},
	iter::Copied,
	slice,
};

pub trait CopyIter<'a> {
	type CopyItem;
	fn citer(&'a self) -> Self::CopyItem;
}

impl<'a, T: Copy + 'a> CopyIter<'a> for Vec<T> {
	type CopyItem = Copied<slice::Iter<'a, T>>;

	fn citer(&'a self) -> Self::CopyItem {
		self.iter().copied()
	}
}

impl<'a, T: Copy + 'a> CopyIter<'a> for VecDeque<T> {
	type CopyItem = Copied<vec_deque::Iter<'a, T>>;

	fn citer(&'a self) -> Self::CopyItem {
		self.iter().copied()
	}
}

impl<'a, T: Copy + 'a> CopyIter<'a> for &[T] {
	type CopyItem = Copied<slice::Iter<'a, T>>;

	fn citer(&'a self) -> Self::CopyItem {
		self.iter().copied()
	}
}

impl<'a, T: Copy + 'a> CopyIter<'a> for &mut [T] {
	type CopyItem = Copied<slice::Iter<'a, T>>;

	fn citer(&'a self) -> Self::CopyItem {
		self.iter().copied()
	}
}

impl<'a, T: Copy + 'a, const N: usize> CopyIter<'a> for [T; N] {
	type CopyItem = Copied<slice::Iter<'a, T>>;

	fn citer(&'a self) -> Self::CopyItem {
		self.iter().copied()
	}
}

impl<'a, K, V> CopyIter<'a> for HashMap<K, V>
where
	K: Copy + 'a,
	V: Copy + 'a,
	// F: FnMut((&K, &V)) -> (K, V),
{
	type CopyItem = HashMapCopyIter<'a, K, V>;

	fn citer(&'a self) -> Self::CopyItem {
		HashMapCopyIter(self.iter())
	}
}

impl<'a, V> CopyIter<'a> for HashSet<V>
where
	V: Copy + 'a,
	// F: FnMut((&K, &V)) -> (K, V),
{
	type CopyItem = Copied<hash_set::Iter<'a, V>>;

	fn citer(&'a self) -> Self::CopyItem {
		self.iter().copied()
	}
}

pub struct HashMapCopyIter<'a, K, V>(hash_map::Iter<'a, K, V>);

impl<'a, K: Copy, V: Copy> Iterator for HashMapCopyIter<'a, K, V> {
	type Item = (K, V);

	fn next(&mut self) -> Option<Self::Item> {
		self.0.next().map(|(&k, &v)| (k, v))
	}
}

pub trait CopyIterKeys<'a> {
	type CopyK;
	fn ckeys(&'a self) -> Self::CopyK;
}

pub trait CopyIterValues<'a> {
	type CopyV;
	fn cvalues(&'a self) -> Self::CopyV;
}

impl<'a, K: 'a + Copy, V: 'a> CopyIterKeys<'a> for HashMap<K, V> {
	type CopyK = Copied<Keys<'a, K, V>>;

	fn ckeys(&'a self) -> Self::CopyK {
		self.keys().copied()
	}
}

impl<'a, K: 'a, V: 'a + Copy> CopyIterValues<'a> for HashMap<K, V> {
	type CopyV = Copied<Values<'a, K, V>>;

	fn cvalues(&'a self) -> Self::CopyV {
		self.values().copied()
	}
}

pub trait MoreItertools: Iterator {
	/// Returns the `n` lowest items in the iterator, in unsorted order.
	fn min_n(mut self, n: usize) -> Vec<Self::Item>
	where
		Self: Sized,
		Self::Item: Ord,
	{
		let first_n = &mut self;
		let mut heap = BinaryHeap::with_capacity(n + 1);
		// Put the first n items in the heap
		heap.extend(first_n.take(n));
		// For the rest, put an item and pop the largest item. This will leave the `n` lowest items
		// in the heap.
		for item in self {
			heap.push(item);
			heap.pop();
		}
		// Return the unsorted `Vec`.
		heap.into_vec()
	}
	/// Returns the `n` lowest items in the iterator, in unsorted order, according to the key.
	fn min_n_by_key<F, C>(self, n: usize, key: F) -> Vec<Self::Item>
	where
		Self: Sized,
		F: Fn(&Self::Item) -> C,
		C: Ord,
	{
		let mut self_comparable = self.map(|item| {
			let comp = key(&item);
			Comparable::new(item, comp)
		});
		let first_n = &mut self_comparable;
		let mut heap = BinaryHeap::with_capacity(n + 1);
		// Put the first n items in the heap
		heap.extend(first_n.take(n));
		// For the rest, put an item and pop the largest item. This will leave the `n` lowest items
		// in the heap.
		for item in self_comparable {
			heap.push(item);
			heap.pop();
		}
		// Return the unsorted `Vec`.
		heap.into_iter().map(Comparable::inner).collect()
	}
}

impl<I: Iterator> MoreItertools for I {}

#[derive(Debug, Default, Clone, Copy)]
pub struct Comparable<T, C>(T, C);

impl<T, C> Comparable<T, C> {
	pub fn new(item: T, comp: C) -> Self {
		Self(item, comp)
	}

	pub fn into_parts(self) -> (T, C) {
		(self.0, self.1)
	}

	pub fn inner(self) -> T {
		self.0
	}
}

impl<T, C: PartialEq> PartialEq for Comparable<T, C> {
	fn eq(&self, other: &Self) -> bool {
		self.1 == other.1
	}
}

impl<T, C: Eq> Eq for Comparable<T, C> {}

impl<T, C: PartialOrd> PartialOrd for Comparable<T, C> {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		self.1.partial_cmp(&other.1)
	}
}

impl<T, C: Ord> Ord for Comparable<T, C> {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.1.cmp(&other.1)
	}
}
