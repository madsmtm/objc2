use std::ptr;

use runtime::{Messageable, Object, Sel, objc_msgSend};

#[unsafe_no_drop_flag]
pub struct Id {
	ptr: *Object,
}

impl Id {
	pub fn nil() -> Id {
		Id { ptr: ptr::null() }
	}

	unsafe fn retain(&self) {
		let retain = Sel::register("retain");
		objc_msgSend(self.as_ptr(), retain);
	}

	unsafe fn release(&self) {
		let release = Sel::register("release");
		objc_msgSend(self.as_ptr(), release);
	}

	pub unsafe fn from_ptr(ptr: *Object) -> Id {
		let id = Id { ptr: ptr };
		id.retain();
		id
	}

	pub unsafe fn from_retained_ptr(ptr: *Object) -> Id {
		Id { ptr: ptr }
	}
}

impl Messageable for Id {
	unsafe fn as_ptr(&self) -> *Object {
		self.ptr
	}
}

impl Clone for Id {
	fn clone(&self) -> Id {
		unsafe {
			Id::from_ptr(self.ptr)
		}
	}
}

impl Drop for Id {
	fn drop(&mut self) {
		if !self.ptr.is_null() {
			unsafe {
				self.release();
			}
			self.ptr = ptr::null();
		}
	}
}

pub trait FromId {
	unsafe fn from_id(id: Id) -> Self;

	unsafe fn from_ptr(ptr: *Object) -> Self {
		FromId::from_id(Id::from_ptr(ptr))
	}

	unsafe fn from_retained_ptr(ptr: *Object) -> Self {
		FromId::from_id(Id::from_retained_ptr(ptr))
	}

	unsafe fn maybe_from_ptr(ptr: *Object) -> Option<Self> {
		if ptr.is_null() {
			None
		} else {
			Some(FromId::from_ptr(ptr))
		}
	}

	unsafe fn maybe_from_retained_ptr(ptr: *Object) -> Option<Self> {
		if ptr.is_null() {
			None
		} else {
			Some(FromId::from_retained_ptr(ptr))
		}
	}
}
