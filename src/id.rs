use std::fmt;
use std::hash;
use std::ptr;

use runtime::{Class, Messageable, Object};
use foundation::INSObject;

#[unsafe_no_drop_flag]
pub struct Id<T> {
	ptr: *T,
}

impl<T> Id<T> {
	pub fn nil() -> Id<T> {
		Id { ptr: ptr::null() }
	}

	unsafe fn retain(&self) {
		msg_send![self.as_ptr() retain];
	}

	unsafe fn release(&self) {
		msg_send![self.as_ptr() release];
	}

	pub unsafe fn from_ptr(ptr: *T) -> Id<T> {
		let id = Id { ptr: ptr };
		id.retain();
		id
	}

	pub unsafe fn from_retained_ptr(ptr: *T) -> Id<T> {
		Id { ptr: ptr }
	}
}

impl<T> Messageable for Id<T> {
	unsafe fn as_ptr(&self) -> *Object {
		self.ptr as *Object
	}
}

impl<T> Clone for Id<T> {
	fn clone(&self) -> Id<T> {
		unsafe {
			Id::from_ptr(self.ptr)
		}
	}
}

#[unsafe_destructor]
impl<T> Drop for Id<T> {
	fn drop(&mut self) {
		if !self.ptr.is_null() {
			unsafe {
				self.release();
			}
			self.ptr = ptr::null();
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

pub struct ClassName<T> {
	name: &'static str,
}

impl<T> ClassName<T> {
	pub fn from_str(name: &'static str) -> ClassName<T> {
		ClassName { name: name }
	}

	pub fn as_str(&self) -> &'static str {
		self.name
	}
}

pub fn class<T: INSObject>() -> Class {
	let name: ClassName<T> = INSObject::class_name();
	Class::get(name.as_str())
}
