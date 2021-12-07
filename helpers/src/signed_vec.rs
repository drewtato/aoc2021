use std::{
	collections::VecDeque,
	mem,
	ops::{Index, IndexMut},
};

/// A `VecDeque` that can be indexed with [`isize`].
///
/// Item `0` is the backing [`VecDeque`], and item `1` is the index of the first element in the
/// `VecDeque`. The index in the `SignedVec` is equal to the index in the `VecDeque` plus item `1`. In
/// other words, to get the index in the `VecDeque`, take the index in the `SignedVec` and subtract
/// item `1`.
///
/// To set the value at an arbitrary index, use [`set`](SignedVec::set), which will fill
/// intermediate items with defaults.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SignedVec<T>(pub VecDeque<T>, pub isize);

impl<T> SignedVec<T> {
	pub fn new() -> Self {
		Self(VecDeque::new(), 0)
	}

	pub fn from_vec_deque(vec_deque: VecDeque<T>) -> Self {
		Self(vec_deque, 0)
	}

	pub fn internal_index(&self, index: isize) -> Result<usize, isize> {
		let int_idx = index - self.1;
		int_idx.try_into().map_err(|_| int_idx)
	}

	pub fn get(&self, index: isize) -> Option<&T> {
		self.internal_index(index)
			.ok()
			.and_then(|ii| self.0.get(ii))
	}
	pub fn get_mut(&mut self, index: isize) -> Option<&mut T> {
		self.internal_index(index)
			.ok()
			.and_then(|ii| self.0.get_mut(ii))
	}

	/// Pushes an item to the front. Unlike [`VecDeque`], this does not affect the indexes of the
	/// other elements.
	pub fn push_front(&mut self, val: T) {
		self.0.push_front(val);
		self.1 -= 1;
	}
	/// Pops an item from the front. Unlike [`VecDeque`], this does not affect the indexes of the
	/// other elements.
	pub fn pop_front(&mut self) -> Option<T> {
		if let Some(v) = self.0.pop_front() {
			self.1 += 1;
			Some(v)
		} else {
			None
		}
	}

	pub fn push_back(&mut self, val: T) {
		self.0.push_back(val);
	}
	pub fn pop_back(&mut self) -> Option<T> {
		self.0.pop_back()
	}

	pub fn iter(&self) -> impl ExactSizeIterator<Item = (isize, &T)> {
		(self.1..(self.1 + self.0.len() as isize)).zip(self.0.iter())
	}
}

impl<T: Default> SignedVec<T> {
	/// Sets an item at an arbitrary index. Returns the old item if one existed.
	pub fn set(&mut self, index: isize, mut val: T) -> Option<T> {
		match self.internal_index(index) {
			Ok(nonneg) => {
				if nonneg >= self.0.len() {
					self.0.resize_with(nonneg, Default::default);
					self.0.push_back(val);
					None
				} else {
					mem::swap(&mut self.0[nonneg], &mut val);
					Some(val)
				}
			}
			Err(neg) => {
				let added = (-neg) as usize;
				self.0
					.resize_with(added + self.0.len() - 1, Default::default);
				self.0.rotate_right(added);
				self.0.push_front(val);
				None
			}
		}
	}
}

impl<T> Index<isize> for SignedVec<T> {
	type Output = T;

	fn index(&self, index: isize) -> &Self::Output {
		self.get(index).unwrap()
	}
}

impl<T> IndexMut<isize> for SignedVec<T> {
	fn index_mut(&mut self, index: isize) -> &mut Self::Output {
		self.get_mut(index).unwrap()
	}
}

/// Uses the existing `VecDeque` to create a draining iterator. If you want to use the `SignedVec`
/// again, clone it first.
///
/// The item is (isize, T), which corresponds to calling `.into_iter().enumerate()` on a normal
/// `Vec`.
impl<T> Iterator for SignedVec<T> {
	type Item = (isize, T);

	fn next(&mut self) -> Option<Self::Item> {
		self.1 += 1;
		self.pop_front().map(|v| (self.1 - 1, v))
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		(self.0.len(), Some(self.0.len()))
	}
}

impl<T> DoubleEndedIterator for SignedVec<T> {
	fn next_back(&mut self) -> Option<Self::Item> {
		self.pop_back().map(|v| (self.0.len() as isize + self.1, v))
	}
}

impl<T> ExactSizeIterator for SignedVec<T> {
	fn len(&self) -> usize {
		self.0.len()
	}
}
