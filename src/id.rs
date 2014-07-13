use std::fmt;
use std::hash;
use std::mem;
use std::ptr;

use runtime::{Class, Messageable, Object};
use foundation::INSObject;

#[unsafe_no_drop_flag]
pub struct Id<T> {
	ptr: *T,
}

impl<T> Id<T> {
	pub unsafe fn from_ptr(ptr: *T) -> Id<T> {
		msg_send![ptr as *Object retain];
		Id::from_retained_ptr(ptr)
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
				msg_send![self release];
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

pub trait IdVector<T> {
	fn as_refs_slice<'a>(&'a self) -> &'a [&'a T];

	fn as_ptrs_slice<'a>(&'a self) -> &'a [*T] {
		unsafe {
			mem::transmute(self.as_refs_slice())
		}
	}
}

impl<T> IdVector<T> for Vec<Id<T>> {
	fn as_refs_slice<'a>(&'a self) -> &'a [&'a T] {
		unsafe {
			mem::transmute(self.as_slice())
		}
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
