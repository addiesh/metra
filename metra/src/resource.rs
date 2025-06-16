use alloc::boxed::Box;
use core::borrow::{Borrow, BorrowMut};
use core::ptr::NonNull;
use log::debug;

/// Represents an allocation of a managed resource, such as a mesh.
/// A resource cannot be copied nor cloned.
pub struct Resource<T> {
	target: *mut ResourceTarget<T>,
}
// SAFETY: resource will always be alive while the handle itself is alive.
impl<T> Borrow<T> for Resource<T> {
	fn borrow(&self) -> &T {
		unsafe {
			self.target
				.as_ref()
				.unwrap_unchecked()
				.data
				.as_ref()
				.expect("resource must not outlive engine")
		}
	}
}
impl<T> BorrowMut<T> for Resource<T> {
	fn borrow_mut(&mut self) -> &mut T {
		unsafe {
			self.target
				.as_mut()
				.unwrap_unchecked()
				.data
				.as_mut()
				.expect("resource must not outlive engine")
		}
	}
}
// The invariants we're managing:
//	- if the game state is dropped but a handle to a resource still exists,
//	  we cannot free its allocation, and must instead mark it as unusable.
// it is possible to get rid of in_use, but only if invariants are carefully managed:
//	- the engine can never mutate Some to None (unless the engine is closing)
//	- the engine will only ever drop None values, and will check BEFORE changing Some to None
// 	- the consumer will NEVER drop None values
// that's complex control flow! But we can do it.
// Also, most of that is overkill! Statics are a stupid idea anyway.
pub(crate) struct ResourceTarget<T> {
	/// This is an Option<T> instead of just T because the engine needs a way of indicating that
	/// the data may no longer exist due to the engine closing.
	pub data: Option<T>,
}
impl<T: 'static> Resource<T> {
	pub(crate) fn new(data: T) -> (Self, NonNull<ResourceTarget<T>>) {
		let target: &'static mut _ = Box::leak(Box::new(ResourceTarget { data: Some(data) }));

		(Self { target }, target.into())
	}
}
impl<T> Drop for Resource<T> {
	fn drop(&mut self) {
		unsafe {
			debug!("dropping resource");
			(*self.target).data = None;
		}
	}
}
