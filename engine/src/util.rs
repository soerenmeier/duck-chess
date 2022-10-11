use std::slice::Iter;

use arrayvec::ArrayVec;


pub struct HighestScoreArray<T, const L: usize> {
	inner: ArrayVec<(f32, T), L>
}

impl<T, const L: usize> HighestScoreArray<T, L> {
	pub fn new() -> Self {
		Self {
			inner: ArrayVec::new()
		}
	}

	pub fn len(&self) -> usize {
		self.inner.len()
	}

	pub fn highest_score(&self) -> Option<f32> {
		self.inner.first().map(|(s, _)| *s)
	}

	pub fn lowest_score(&self) -> Option<f32> {
		self.inner.last().map(|(s, _)| *s)
	}

	pub fn should_insert(&self, score: f32) -> bool {
		score > self.lowest_score().unwrap_or(f32::MIN) ||
		!self.inner.is_full()
	}

	fn sort(&mut self) {
		self.inner.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap().reverse());
	}

	/// ## Panics
	/// if the score cannot be compared
	pub fn insert(&mut self, score: f32, val: T) {
		if !self.should_insert(score) {
			return
		}

		if self.inner.is_full() {
			self.inner.pop();
		}
		self.inner.push((score, val));
		self.sort();
	}

	pub fn iter(&self) -> Iter<'_, (f32, T)> {
		self.inner.iter()
	}
}

impl<T: Clone, const L: usize> HighestScoreArray<T, L> {
	pub fn to_vec(&self) -> Vec<(f32, T)> {
		self.inner.to_vec()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_fn() {
		let mut list: HighestScoreArray<_, 3> = HighestScoreArray::new();
		list.insert(0f32, ());
		assert_eq!(list.lowest_score().unwrap(), 0f32);
		list.insert(-3f32, ());
		assert_eq!(list.lowest_score().unwrap(), -3f32);
		list.insert(5f32, ());
		assert_eq!(list.lowest_score().unwrap(), -3f32);
		list.insert(-8f32, ());
		assert_eq!(list.lowest_score().unwrap(), -3f32);
	}
}