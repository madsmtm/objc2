use std::fmt;
use std::hash;
use std::mem;

use runtime::{Message, Object};
use {ToMessage};

#[unsafe_no_drop_flag]
pub struct Id<T> {
	ptr: *mut T,
}

impl<T: Message> Id<T> {
	pub unsafe fn from_ptr(ptr: *mut T) -> Id<T> {
		match Id::maybe_from_ptr(ptr) {
			Some(id) => id,
			None => fail!("Attempted to construct an Id from a null pointer"),
		}
	}

	pub unsafe fn from_retained_ptr(ptr: *mut T) -> Id<T> {
		match Id::maybe_from_retained_ptr(ptr) {
			Some(id) => id,
			None => fail!("Attempted to construct an Id from a null pointer"),
		}
	}

	pub unsafe fn maybe_from_ptr(ptr: *mut T) -> Option<Id<T>> {
		// objc_msgSend is a no-op on null pointers
		msg_send![ptr retain];
		Id::maybe_from_retained_ptr(ptr)
	}

	pub unsafe fn maybe_from_retained_ptr(ptr: *mut T) -> Option<Id<T>> {
		if ptr.is_null() {
			None
		} else {
			Some(Id { ptr: ptr })
		}
	}
}

impl<T: Message> ToMessage for Id<T> {
	fn as_ptr(&self) -> *mut Object {
		self.ptr.as_ptr()
	}
}

#[unsafe_destructor]
impl<T: Message> Drop for Id<T> {
	fn drop(&mut self) {
		if !self.ptr.is_null() {
			let ptr = mem::replace(&mut self.ptr, RawPtr::null());
			unsafe {
				msg_send![ptr release];
			}
		}
	}
}

impl<T> Deref<T> for Id<T> {
	fn deref(&self) -> &T {
		unsafe { &*self.ptr }
	}
}

impl<T: PartialEq> PartialEq for Id<T> {
	fn eq(&self, other: &Id<T>) -> bool {
		self.deref() == other.deref()
	}

	fn ne(&self, other: &Id<T>) -> bool {
		self.deref() != other.deref()
	}
}

impl<T: Eq> Eq for Id<T> { }

impl<T: hash::Hash> hash::Hash for Id<T> {
	fn hash(&self, state: &mut hash::sip::SipState) {
		self.deref().hash(state)
	}
}

impl<T: fmt::Show> fmt::Show for Id<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.deref().fmt(f)
	}
}

pub trait IdVector<T> {
	fn as_refs_slice(&self) -> &[&T];
}

impl<T, V: Slice<Id<T>>> IdVector<T> for V {
	fn as_refs_slice(&self) -> &[&T] {
		unsafe {
			mem::transmute(self.as_slice())
		}
	}
}

pub trait IntoIdVector<T> {
	unsafe fn into_id_vec(self) -> Vec<Id<T>>;
}

impl<T: Message> IntoIdVector<T> for Vec<*mut T> {
	unsafe fn into_id_vec(self) -> Vec<Id<T>> {
		for &ptr in self.iter() {
			msg_send![ptr retain];
		}
		mem::transmute(self)
	}
}

impl<'a, T: Message> IntoIdVector<T> for Vec<&'a T> {
	unsafe fn into_id_vec(self) -> Vec<Id<T>> {
		for &obj in self.iter() {
			msg_send![obj retain];
		}
		mem::transmute(self)
	}
}
