use std::cell::UnsafeCell;

use {Identifier, ShareId, ToMessage};
use runtime::{Message, Object};

#[link(name = "Foundation", kind = "framework")]
extern {
	fn objc_storeWeak(location: *mut *mut Object, obj: *mut Object) -> *mut Object;
	fn objc_loadWeakRetained(location: *mut *mut Object) -> *mut Object;
	fn objc_destroyWeak(addr: *mut *mut Object);
}

pub struct WeakId<T> {
	// Our pointer must have the same address even if we are moved, so Box it.
	// Although loading the WeakId may modify the pointer, it is thread safe,
	// so we must use an UnsafeCell to get a *mut without self being mutable.
	ptr: Box<UnsafeCell<*const T>>,
}

impl<T: Message> WeakId<T> {
	pub fn new(obj: &ShareId<T>) -> WeakId<T> {
		let ptr = box UnsafeCell::new(RawPtr::null());
		unsafe {
			let loc = ptr.get() as *mut *mut Object;
			objc_storeWeak(loc, obj.as_ptr());
		}
		WeakId { ptr: ptr }
	}

	pub fn load(&self) -> Option<ShareId<T>> {
		unsafe {
			let loc = self.ptr.get() as *mut *mut Object;
			let obj = objc_loadWeakRetained(loc);
			Identifier::maybe_from_retained_ptr(obj as *mut T)
		}
	}
}

#[unsafe_destructor]
impl<T> Drop for WeakId<T> {
	fn drop(&mut self) {
		unsafe {
			let loc = self.ptr.get() as *mut *mut Object;
			objc_destroyWeak(loc);
		}
	}
}

#[cfg(test)]
mod tests {
	use {Id};
	use foundation::{INSObject, NSObject};
	use super::WeakId;

	#[test]
	fn test_weak() {
		let obj: Id<NSObject> = INSObject::new();
		let obj = obj.share();

		let weak = WeakId::new(&obj);
		let strong = weak.load().unwrap();
		assert!(strong == obj);
		drop(strong);

		drop(obj);
		assert!(weak.load().is_none());
	}
}
