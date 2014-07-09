use std::mem;

use runtime::Messageable;
use id::{class, Id, IdVector};
use super::{INSCopying, INSObject};

/*
pub trait INSEnumerator<T: FromId> : INSObject {
	fn next_object(&mut self) -> Option<T> {
		unsafe {
			let obj = msg_send![self.as_ptr() nextObject];
			FromId::maybe_from_ptr(obj)
		}
	}
}

#[deriving(Clone)]
pub struct NSEnumerator<'a, T> {
	ptr: Id,
}

impl<'a, T> Messageable for NSEnumerator<'a, T> {
	unsafe fn as_ptr(&self) -> *Object {
		self.ptr.as_ptr()
	}
}

impl<'a, T> FromId for NSEnumerator<'a, T> {
	unsafe fn from_id(id: Id) -> NSEnumerator<'a, T> {
		NSEnumerator { ptr: id }
	}
}

impl<'a, T> INSObject for NSEnumerator<'a, T> {
	fn class_name() -> ClassName<NSEnumerator<'a, T>> {
		ClassName::from_str("NSEnumerator")
	}
}

impl<'a, T: FromId> INSEnumerator<T> for NSEnumerator<'a, T> { }

impl<'a, T: FromId + Messageable> Iterator<T> for NSEnumerator<'a, T> {
	fn next(&mut self) -> Option<T> {
		self.next_object()
	}
}
*/

pub trait INSArray<T: INSObject> : INSObject {
	fn count(&self) -> uint {
		let result = unsafe {
			msg_send![self.as_ptr() count]
		};
		result as uint
	}

	fn object_at<'a>(&'a self, index: uint) -> &'a T {
		unsafe {
			let result = msg_send![self.as_ptr() objectAtIndex:index];
			mem::transmute(result)
		}
	}

/*
	fn object_enumerator<'a>(&'a self) -> NSEnumerator<'a, T> {
		unsafe {
			let result = msg_send![self.as_ptr() objectEnumerator];
			FromId::from_ptr(result)
		}
	}
*/

	unsafe fn from_refs(refs: &[&T]) -> Id<Self> {
		let cls = class::<Self>();
		let obj = msg_send![cls.as_ptr() alloc];
		let obj = msg_send![obj initWithObjects:refs.as_ptr() count:refs.len()];
		Id::from_retained_ptr(obj as *Self)
	}

	fn from_vec(vec: Vec<Id<T>>) -> Id<Self> {
		let refs = vec.as_refs_slice();
		unsafe {
			INSArray::from_refs(refs)
		}
	}
}

object_struct!(NSArray<T>)

impl<T: INSObject> INSArray<T> for NSArray<T> { }

impl<T> INSCopying<NSArray<T>> for NSArray<T> { }

impl<T: INSObject> Collection for NSArray<T> {
	fn len(&self) -> uint {
		self.count()
	}
}
