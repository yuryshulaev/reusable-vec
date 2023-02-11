pub struct ReusableVec<T> {
	vec: Vec<T>,
	len: usize,
}

impl<T> ReusableVec<T> {
	#[inline]
	pub fn new() -> Self {
		Self { vec: Vec::new(), len: 0 }
	}

	#[inline]
	pub fn with_capacity(capacity: usize) -> Self {
		Self { vec: Vec::with_capacity(capacity), len: 0 }
	}

	#[inline]
	pub fn push_reuse(&mut self) -> Option<&mut T> {
		(self.len < self.vec.len()).then(move || {
			self.len += 1;
			&mut self.vec[self.len - 1]
		})
	}

	#[inline]
	pub fn push(&mut self, value: T) {
		self.len = self.len.checked_add(1).unwrap();

		if self.len <= self.vec.len() {
			self.vec[self.len - 1] = value;
		} else {
			self.vec.push(value);
		}
	}

	#[inline]
	pub fn as_slice(&self) -> &[T] {
		&self.vec[..self.len]
	}

	#[inline]
	pub fn as_mut_slice(&mut self) -> &mut [T] {
		&mut self.vec[..self.len]
	}

	#[inline]
	pub fn into_vec(mut self) -> Vec<T> {
		self.vec.truncate(self.len);
		self.vec
	}

	#[inline]
	pub fn clear_reuse(&mut self) {
		self.len = 0;
	}

	#[inline]
	pub fn clear_drop(&mut self) {
		self.vec.clear();
		self.len = 0;
	}
}

impl<T> From<Vec<T>> for ReusableVec<T> {
	#[inline]
	fn from(vec: Vec<T>) -> Self {
		Self { len: vec.len(), vec }
	}
}

impl<T> From<ReusableVec<T>> for Vec<T> {
	#[inline]
	fn from(reusable: ReusableVec<T>) -> Vec<T> {
		reusable.into_vec()
	}
}

impl<T> std::ops::Deref for ReusableVec<T> {
	type Target = [T];

	#[inline]
	fn deref(&self) -> &Self::Target {
		self.as_slice()
	}
}

impl<T> std::ops::DerefMut for ReusableVec<T> {
	#[inline]
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.as_mut_slice()
	}
}

impl<T> IntoIterator for ReusableVec<T> {
	type Item = T;
	type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

	#[inline]
	fn into_iter(self) -> Self::IntoIter {
		self.into_vec().into_iter()
	}
}

impl<'a, T> IntoIterator for &'a ReusableVec<T> {
	type Item = &'a T;
	type IntoIter = <&'a [T] as IntoIterator>::IntoIter;

	#[inline]
	fn into_iter(self) -> Self::IntoIter {
		self.as_slice().into_iter()
	}
}

impl<'a, T> IntoIterator for &'a mut ReusableVec<T> {
	type Item = &'a mut T;
	type IntoIter = <&'a mut [T] as IntoIterator>::IntoIter;

	#[inline]
	fn into_iter(self) -> Self::IntoIter {
		self.as_mut_slice().iter_mut()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_should_work() {
		struct Thing {
			cheap: u32,
			expensive: Vec<u32>,
		}

		let mut things = ReusableVec::<Thing>::new();

		for i in 0..3 {
			let new_thing = Thing { cheap: 123, expensive: Vec::new() };

			if let Some(reused) = things.push_reuse() {
				let mut expensive = std::mem::take(&mut reused.expensive);

				if i > 0 {
					assert_eq!(expensive, [456]);
				}

				expensive.clear();
				*reused = Thing { expensive, ..new_thing };
			} else {
				things.push(Thing { expensive: Vec::with_capacity(100), ..new_thing });
			}

			assert_eq!(things.len(), 1);
			let last = things.last_mut().unwrap();
			last.expensive.push(456);

			assert_eq!(last.cheap, 123);
			assert_eq!(last.expensive, [456]);

			things.clear_reuse();
			assert_eq!(things.len(), 0);
		}

		things.clear_drop();
		assert_eq!(things.len(), 0);
		assert!(things.push_reuse().is_none());

		things.push(Thing { cheap: 0, expensive: Vec::new() });
		things.clear_reuse();
		assert_eq!(Vec::from(things).len(), 0);
	}
}
