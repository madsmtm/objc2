use std::kinds::marker::ContravariantLifetime;
use std::mem;

use runtime::Object;
use {class, Id, IdVector};
use super::{INSCopying, INSObject};

pub struct NSEnumerator<'a, T> {
	id: Id<Object>,
	marker: ContravariantLifetime<'a>,
}

impl<'a, T> NSEnumerator<'a, T> {
	unsafe fn from_ptr(ptr: *Object) -> NSEnumerator<'a, T> {
		NSEnumerator { id: Id::from_ptr(ptr), marker: ContravariantLifetime }
	}
}

impl<'a, T> Iterator<&'a T> for NSEnumerator<'a, T> {
	fn next(&mut self) -> Option<&'a T> {
		unsafe {
			let obj = msg_send![self.id nextObject];
			if obj.is_null() {
				None
			} else {
				Some(mem::transmute(obj))
			}
		}
	}
}

pub trait INSArray<T: INSObject> : INSObject {
	fn count(&self) -> uint {
		let result = unsafe {
			msg_send![self count]
		};
		result as uint
	}

	fn object_at<'a>(&'a self, index: uint) -> &'a T {
		unsafe {
			let result = msg_send![self objectAtIndex:index];
			mem::transmute(result)
		}
	}

	fn object_enumerator<'a>(&'a self) -> NSEnumerator<'a, T> {
		unsafe {
			let result = msg_send![self objectEnumerator];
			NSEnumerator::from_ptr(result)
		}
	}

	unsafe fn from_refs(refs: &[&T]) -> Id<Self> {
		let cls = class::<Self>();
		let obj = msg_send![cls alloc];
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
