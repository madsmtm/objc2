use std::fmt;
use std::hash;
use std::mem;

use runtime::{Message, Object};
use {ToMessage};

pub trait Identifier<T: Message> {
	unsafe fn maybe_from_retained_ptr(ptr: *mut T) -> Option<Self>;

	unsafe fn maybe_from_ptr(ptr: *mut T) -> Option<Self> {
		// objc_msgSend is a no-op on null pointers
		msg_send![ptr retain];
		Identifier::maybe_from_retained_ptr(ptr)
	}

	unsafe fn from_retained_ptr(ptr: *mut T) -> Self {
		match Identifier::maybe_from_retained_ptr(ptr) {
			Some(id) => id,
			None => fail!("Attempted to construct an Id from a null pointer"),
		}
	}

	unsafe fn from_ptr(ptr: *mut T) -> Self {
		match Identifier::maybe_from_ptr(ptr) {
			Some(id) => id,
			None => fail!("Attempted to construct an Id from a null pointer"),
		}
	}
}

#[unsafe_no_drop_flag]
pub struct Id<T> {
	ptr: *mut T,
}

impl<T: Message> Identifier<T> for Id<T> {
	unsafe fn maybe_from_retained_ptr(ptr: *mut T) -> Option<Id<T>> {
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

impl<T, I: Identifier<T>, V: Vector<I>> IdVector<T> for V {
	fn as_refs_slice(&self) -> &[&T] {
		unsafe {
			mem::transmute(self.as_slice())
		}
	}
}

pub trait IntoIdVector<T, I: Identifier<T>> {
	unsafe fn into_id_vec(self) -> Vec<I>;
}

impl<T: Message, I: Identifier<T>> IntoIdVector<T, I> for Vec<*mut T> {
	unsafe fn into_id_vec(self) -> Vec<I> {
		for &ptr in self.iter() {
			msg_send![ptr retain];
		}
		mem::transmute(self)
	}
}

impl<'a, T: Message, I: Identifier<T>> IntoIdVector<T, I> for Vec<&'a T> {
	unsafe fn into_id_vec(self) -> Vec<I> {
		for &obj in self.iter() {
			msg_send![obj retain];
		}
		mem::transmute(self)
	}
}
