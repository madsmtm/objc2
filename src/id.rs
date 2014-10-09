use std::fmt;
use std::hash;
use std::mem;

use runtime::{Message, Object};
use {ToMessage};

pub struct Owned;
pub struct Shared;

pub trait Ownership { }
impl Ownership for Owned { }
impl Ownership for Shared { }

#[unsafe_no_drop_flag]
pub struct Id<T: Message, O: Ownership = Owned> {
	ptr: *mut T,
}

impl<T: Message, O: Ownership> Id<T, O> {
	pub unsafe fn from_ptr(ptr: *mut T) -> Id<T, O> {
		match Id::maybe_from_ptr(ptr) {
			Some(id) => id,
			None => fail!("Attempted to construct an Id from a null pointer"),
		}
	}

	pub unsafe fn from_retained_ptr(ptr: *mut T) -> Id<T, O> {
		match Id::maybe_from_retained_ptr(ptr) {
			Some(id) => id,
			None => fail!("Attempted to construct an Id from a null pointer"),
		}
	}

	pub unsafe fn maybe_from_ptr(ptr: *mut T) -> Option<Id<T, O>> {
		// objc_msgSend is a no-op on null pointers
		msg_send![ptr retain];
		Id::maybe_from_retained_ptr(ptr)
	}

	pub unsafe fn maybe_from_retained_ptr(ptr: *mut T) -> Option<Id<T, O>> {
		if ptr.is_null() {
			None
		} else {
			Some(Id { ptr: ptr })
		}
	}
}

impl<T: Message> Id<T, Owned> {
	pub fn share(self) -> ShareId<T> {
		unsafe {
			mem::transmute(self)
		}
	}
}

impl<T: Message, O: Ownership> ToMessage for Id<T, O> {
	fn as_ptr(&self) -> *mut Object {
		self.ptr.as_ptr()
	}
}

impl<T: Message> Clone for Id<T, Shared> {
	fn clone(&self) -> ShareId<T> {
		unsafe { Id::from_ptr(self.ptr) }
	}
}

#[unsafe_destructor]
impl<T: Message, O: Ownership> Drop for Id<T, O> {
	fn drop(&mut self) {
		if !self.ptr.is_null() {
			let ptr = mem::replace(&mut self.ptr, RawPtr::null());
			unsafe {
				msg_send![ptr release];
			}
		}
	}
}

impl<T: Message, O: Ownership> Deref<T> for Id<T, O> {
	fn deref(&self) -> &T {
		unsafe { &*self.ptr }
	}
}

impl<T: Message> DerefMut<T> for Id<T, Owned> {
	fn deref_mut(&mut self) -> &mut T {
		unsafe { &mut *self.ptr }
	}
}

impl<T: Message + PartialEq, O: Ownership> PartialEq for Id<T, O> {
	fn eq(&self, other: &Id<T, O>) -> bool {
		self.deref() == other.deref()
	}

	fn ne(&self, other: &Id<T, O>) -> bool {
		self.deref() != other.deref()
	}
}

impl<T: Message + Eq, O: Ownership> Eq for Id<T, O> { }

impl<T: Message + hash::Hash, O: Ownership> hash::Hash for Id<T, O> {
	fn hash(&self, state: &mut hash::sip::SipState) {
		self.deref().hash(state)
	}
}

impl<T: Message + fmt::Show, O: Ownership> fmt::Show for Id<T, O> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.deref().fmt(f)
	}
}

pub type ShareId<T> = Id<T, Shared>;

pub trait IdVector<T> for Sized? {
	fn as_refs_slice(&self) -> &[&T];
}

impl<T: Message, O: Ownership> IdVector<T> for [Id<T, O>] {
	fn as_refs_slice(&self) -> &[&T] {
		unsafe {
			mem::transmute(self)
		}
	}
}

impl<T: Message, O: Ownership> IdVector<T> for Vec<Id<T, O>> {
	fn as_refs_slice(&self) -> &[&T] {
		self.as_slice().as_refs_slice()
	}
}

pub trait IntoIdVector<T> {
	unsafe fn into_id_vec<O: Ownership>(self) -> Vec<Id<T, O>>;
}

impl<T: Message> IntoIdVector<T> for Vec<*mut T> {
	unsafe fn into_id_vec<O: Ownership>(self) -> Vec<Id<T, O>> {
		for &ptr in self.iter() {
			msg_send![ptr retain];
		}
		mem::transmute(self)
	}
}

impl<'a, T: Message> IntoIdVector<T> for Vec<&'a T> {
	unsafe fn into_id_vec<O: Ownership>(self) -> Vec<Id<T, O>> {
		for &obj in self.iter() {
			msg_send![obj retain];
		}
		mem::transmute(self)
	}
}
