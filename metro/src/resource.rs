use core::ops::{Deref, DerefMut};

use alloc::boxed::Box;

pub struct Resource<T> {
	target: *mut ResourceTarget<T>,
}
// SAFETY: resource will always be alive while the handle itself is alive.
impl<T> Deref for Resource<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		unsafe {
			self.target
				.as_ref()
				.unwrap_unchecked()
				.data
				.as_ref()
				.unwrap()
		}
	}
}
impl<T> DerefMut for Resource<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe {
			self.target
				.as_mut()
				.unwrap_unchecked()
				.data
				.as_mut()
				.unwrap()
		}
	}
}
pub(crate) struct ResourceTarget<T> {
	/// True if the consumer is tracking the resource.
	pub in_use: bool,
	pub data: Option<T>,
}
impl<T> Resource<T> {
	pub(crate) fn new(data: T) -> Self {
		Self {
			target: Box::leak(Box::new(ResourceTarget {
				in_use: true,
				data: Some(data),
			})),
		}
	}
}
impl<T> Drop for Resource<T> {
	fn drop(&mut self) {
		unsafe { drop(Box::from_raw(self.target)) }
	}
}
