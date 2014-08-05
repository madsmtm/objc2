use std::fmt;
use std::hash;
use std::mem;
use std::ptr;

use runtime::{Message, Object, ToMessage};

#[unsafe_no_drop_flag]
pub struct Id<T> {
	ptr: *mut T,
}

impl<T: Message> Id<T> {
	pub unsafe fn from_ptr(ptr: *mut T) -> Id<T> {
		msg_send![ptr retain];
		Id::from_retained_ptr(ptr)
	}

	pub unsafe fn from_retained_ptr(ptr: *mut T) -> Id<T> {
		Id { ptr: ptr }
	}
}

impl<T: Message> ToMessage for Id<T> {
	fn as_ptr(&self) -> *mut Object {
		self.ptr.as_ptr()
	}
}

impl<T: Message> Clone for Id<T> {
	fn clone(&self) -> Id<T> {
		unsafe {
			Id::from_ptr(self.ptr)
		}
	}
}

#[unsafe_destructor]
impl<T: Message> Drop for Id<T> {
	fn drop(&mut self) {
		if !self.ptr.is_null() {
			let ptr = mem::replace(&mut self.ptr, ptr::mut_null());
			unsafe {
				msg_send![ptr release];
			}
		}
	}
}

impl<T> Deref<T> for Id<T> {
	fn deref<'a>(&'a self) -> &'a T {
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

impl<S: hash::Writer, T: hash::Hash<S>> hash::Hash<S> for Id<T> {
	fn hash(&self, state: &mut S) {
		self.deref().hash(state)
	}
}

impl<T: fmt::Show> fmt::Show for Id<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.deref().fmt(f)
	}
}

pub trait IdVector<T> {
	fn as_refs_slice<'a>(&'a self) -> &'a [&'a T];
}

impl<T, V: Vector<Id<T>>> IdVector<T> for V {
	fn as_refs_slice<'a>(&'a self) -> &'a [&'a T] {
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
