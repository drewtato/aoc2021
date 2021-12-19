use std::cmp::Ordering;
use std::collections::HashMap;
use std::num::Wrapping;
use std::ops::{Index, IndexMut};

use num_traits::AsPrimitive;

/// Wraps the type in [`Wrapping`] to allow wrapping arithmetic by default. Use `.0` to access inner
/// value.
pub const fn w<T>(t: T) -> Wrapping<T> {
	Wrapping(t)
}

pub fn idx<N>(index: N) -> usize
where
	N: AsPrimitive<usize>,
{
	index.as_()
}

pub fn widx<N>(index: Wrapping<N>) -> usize
where
	N: AsPrimitive<usize>,
{
	index.0.as_()
}

pub trait AsUsizeIndex<I>: Index<usize> {
	fn i(&self, index: I) -> &Self::Output;
}

pub trait AsUsizeIndexMut<I>: IndexMut<usize> {
	fn im(&mut self, index: I) -> &mut Self::Output;
}

macro_rules! impl_as_usize_index {
	($($int_type:ty)*) => {
		$(
			impl<T> AsUsizeIndex<$int_type> for T
			where T: Index<usize>
			{
				fn i(&self, index: $int_type) -> &Self::Output {
					self.index(index as usize)
				}
			}
			impl<T> AsUsizeIndexMut<$int_type> for T
			where T: IndexMut<usize>
			{
				fn im(&mut self, index: $int_type) -> &mut Self::Output {
					self.index_mut(index as usize)
				}
			}

			impl<T> AsUsizeIndex<Wrapping<$int_type>> for T
			where T: Index<usize>
			{
				fn i(&self, index: Wrapping<$int_type>) -> &Self::Output {
					self.index(index.0 as usize)
				}
			}
			impl<T> AsUsizeIndexMut<Wrapping<$int_type>> for T
			where T: IndexMut<usize>
			{
				fn im(&mut self, index: Wrapping<$int_type>) -> &mut Self::Output {
					self.index_mut(index.0 as usize)
				}
			}
		)*
	};
}

impl_as_usize_index! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct ArrayMapIter<'a, V, A> {
	inner: &'a V,
	y: Wrapping<usize>,
	x: Wrapping<usize>,
	arr: A,
	progress: usize,
}

impl<'a, V, A> ArrayMapIter<'a, V, A> {
	pub fn new<N, M>(inner: &'a V, y: N, x: M, arr: A) -> Self
	where
		N: AsPrimitive<usize>,
		M: AsPrimitive<usize>,
	{
		Self {
			inner,
			y: w(idx(y)),
			x: w(idx(x)),
			arr,
			progress: 0,
		}
	}
}

impl<'a, T> Iterator for ArrayMapIter<'a, Vec<Vec<T>>, &'static [[usize; 2]]> {
	type Item = &'a T;

	fn next(&mut self) -> Option<Self::Item> {
		loop {
			let &[dy, dx] = self.arr.get(self.progress)?;
			let [dy, dx] = [w(dy), w(dx)];
			self.progress += 1;
			let item = self
				.inner
				.get((self.y + dy).0)
				.and_then(|row| row.get((self.x + dx).0));
			if item.is_some() {
				return item;
			}
		}
	}
}

impl<'a, T, const N: usize> Iterator for ArrayMapIter<'a, &[[T; N]], &'static [[usize; 2]]> {
	type Item = &'a T;

	fn next(&mut self) -> Option<Self::Item> {
		loop {
			let &[dy, dx] = self.arr.get(self.progress)?;
			let [dy, dx] = [w(dy), w(dx)];
			self.progress += 1;
			let item = self
				.inner
				.get((self.y + dy).0)
				.and_then(|row| row.get((self.x + dx).0));
			if item.is_some() {
				return item;
			}
		}
	}
}

impl<'a, T> Iterator for ArrayMapIter<'a, HashMap<[usize; 2], T>, &'static [[usize; 2]]> {
	type Item = &'a T;

	fn next(&mut self) -> Option<Self::Item> {
		loop {
			let &[dy, dx] = self.arr.get(self.progress)?;
			let [dy, dx] = [w(dy), w(dx)];
			self.progress += 1;
			let item = self.inner.get(&[(self.y + dy).0, (self.x + dx).0]);
			if item.is_some() {
				return item;
			}
		}
	}
}

pub const ALL_NEIGHBORS: [[usize; 2]; 8] = [
	[usize::MAX, 0],
	[usize::MAX, 1],
	[0, 1],
	[1, 1],
	[1, 0],
	[1, usize::MAX],
	[0, usize::MAX],
	[usize::MAX, usize::MAX],
];

pub const CORNER_NEIGHBORS: [[usize; 2]; 4] = [
	[usize::MAX, 1],
	[1, 1],
	[1, usize::MAX],
	[usize::MAX, usize::MAX],
];

#[rustfmt::skip]
pub const SIDE_NEIGHBORS: [[usize; 2]; 4] = [
	[usize::MAX, 0],
	[0, 1],
	[1, 0],
	[0, usize::MAX],
];

pub trait NeighborIter<'a> {
	type Iter;

	fn array_neighbors<N, M>(&'a self, arr: &'static [[usize; 2]], y: N, x: M) -> Self::Iter
	where
		N: AsPrimitive<usize>,
		M: AsPrimitive<usize>;
	fn side_neighbors<N, M>(&'a self, y: N, x: M) -> Self::Iter
	where
		N: AsPrimitive<usize>,
		M: AsPrimitive<usize>,
	{
		self.array_neighbors(&SIDE_NEIGHBORS, y, x)
	}
	fn corner_neighbors<N, M>(&'a self, y: N, x: M) -> Self::Iter
	where
		N: AsPrimitive<usize>,
		M: AsPrimitive<usize>,
	{
		self.array_neighbors(&CORNER_NEIGHBORS, y, x)
	}
	fn all_neighbors<N, M>(&'a self, y: N, x: M) -> Self::Iter
	where
		N: AsPrimitive<usize>,
		M: AsPrimitive<usize>,
	{
		self.array_neighbors(&ALL_NEIGHBORS, y, x)
	}
}

impl<'a, T: 'a> NeighborIter<'a> for Vec<Vec<T>> {
	type Iter = ArrayMapIter<'a, Self, &'static [[usize; 2]]>;

	fn array_neighbors<N, M>(&'a self, arr: &'static [[usize; 2]], y: N, x: M) -> Self::Iter
	where
		N: AsPrimitive<usize>,
		M: AsPrimitive<usize>,
	{
		ArrayMapIter::new(self, y, x, arr)
	}
}

impl<'a, T: 'a> NeighborIter<'a> for HashMap<[usize; 2], T> {
	type Iter = ArrayMapIter<'a, Self, &'static [[usize; 2]]>;

	fn array_neighbors<N, M>(&'a self, arr: &'static [[usize; 2]], y: N, x: M) -> Self::Iter
	where
		N: AsPrimitive<usize>,
		M: AsPrimitive<usize>,
	{
		ArrayMapIter::new(self, y, x, arr)
	}
}

pub trait IntoSorted {
	type Item;
	fn into_sorted(self) -> Self;
	fn into_sorted_unstable(self) -> Self;
	fn into_sorted_by<F>(self, f: F) -> Self
	where
		F: FnMut(&Self::Item, &Self::Item) -> Ordering;
	fn into_sorted_unstable_by<F>(self, f: F) -> Self
	where
		F: FnMut(&Self::Item, &Self::Item) -> Ordering;
	fn into_sorted_by_key<F, K>(self, f: F) -> Self
	where
		F: FnMut(&Self::Item) -> K,
		K: Ord;
	fn into_sorted_unstable_by_key<F, K>(self, f: F) -> Self
	where
		F: FnMut(&Self::Item) -> K,
		K: Ord;
}

impl<T: Ord> IntoSorted for Vec<T> {
	type Item = T;
	fn into_sorted(mut self) -> Self {
		self.sort();
		self
	}
	fn into_sorted_unstable(mut self) -> Self {
		self.sort_unstable();
		self
	}
	fn into_sorted_by<F>(mut self, f: F) -> Self
	where
		F: FnMut(&Self::Item, &Self::Item) -> Ordering,
	{
		self.sort_by(f);
		self
	}
	fn into_sorted_unstable_by<F>(mut self, f: F) -> Self
	where
		F: FnMut(&Self::Item, &Self::Item) -> Ordering,
	{
		self.sort_unstable_by(f);
		self
	}
	fn into_sorted_by_key<F, K>(mut self, f: F) -> Self
	where
		F: FnMut(&Self::Item) -> K,
		K: Ord,
	{
		self.sort_by_key(f);
		self
	}
	fn into_sorted_unstable_by_key<F, K>(mut self, f: F) -> Self
	where
		F: FnMut(&Self::Item) -> K,
		K: Ord,
	{
		self.sort_unstable_by_key(f);
		self
	}
}

impl<T: Ord, const N: usize> IntoSorted for [T; N] {
	type Item = T;
	fn into_sorted(mut self) -> Self {
		self.sort();
		self
	}
	fn into_sorted_unstable(mut self) -> Self {
		self.sort_unstable();
		self
	}
	fn into_sorted_by<F>(mut self, f: F) -> Self
	where
		F: FnMut(&Self::Item, &Self::Item) -> Ordering,
	{
		self.sort_by(f);
		self
	}
	fn into_sorted_unstable_by<F>(mut self, f: F) -> Self
	where
		F: FnMut(&Self::Item, &Self::Item) -> Ordering,
	{
		self.sort_unstable_by(f);
		self
	}
	fn into_sorted_by_key<F, K>(mut self, f: F) -> Self
	where
		F: FnMut(&Self::Item) -> K,
		K: Ord,
	{
		self.sort_by_key(f);
		self
	}
	fn into_sorted_unstable_by_key<F, K>(mut self, f: F) -> Self
	where
		F: FnMut(&Self::Item) -> K,
		K: Ord,
	{
		self.sort_unstable_by_key(f);
		self
	}
}

impl<T: Ord> IntoSorted for &mut [T] {
	type Item = T;
	fn into_sorted(self) -> Self {
		self.sort();
		self
	}
	fn into_sorted_unstable(self) -> Self {
		self.sort_unstable();
		self
	}
	fn into_sorted_by<F>(self, f: F) -> Self
	where
		F: FnMut(&Self::Item, &Self::Item) -> Ordering,
	{
		self.sort_by(f);
		self
	}
	fn into_sorted_unstable_by<F>(self, f: F) -> Self
	where
		F: FnMut(&Self::Item, &Self::Item) -> Ordering,
	{
		self.sort_unstable_by(f);
		self
	}
	fn into_sorted_by_key<F, K>(self, f: F) -> Self
	where
		F: FnMut(&Self::Item) -> K,
		K: Ord,
	{
		self.sort_by_key(f);
		self
	}
	fn into_sorted_unstable_by_key<F, K>(self, f: F) -> Self
	where
		F: FnMut(&Self::Item) -> K,
		K: Ord,
	{
		self.sort_unstable_by_key(f);
		self
	}
}
