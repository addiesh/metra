use core::ops::Deref;

use alloc::rc::Rc;

pub struct Asset<T> {
	rc: Rc<T>,
}
impl<T> Clone for Asset<T> {
	fn clone(&self) -> Self {
		Self {
			rc: self.rc.clone(),
		}
	}
}
impl<T> Deref for Asset<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		self.rc.deref()
	}
}
impl<T> Asset<T> {
	#[inline]
	pub(crate) fn new(value: T) -> Self {
		Self { rc: Rc::new(value) }
	}
}
