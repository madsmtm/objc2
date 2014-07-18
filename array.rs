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
			let obj = msg_send![self.id nextObject] as *T;
			obj.to_option()
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
			let result = msg_send![self objectAtIndex:index] as *T;
			&*result
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

#[cfg(test)]
mod tests {
	use {Id};
	use foundation::{INSObject, NSObject};
	use super::{INSArray, NSArray};

	#[test]
	fn test_count() {
		let empty_array: Id<NSArray<NSObject>> = INSObject::new();
		assert!(empty_array.count() == 0);

		let vec: Vec<Id<NSObject>> = Vec::from_fn(4, |_| INSObject::new());
		let array: Id<NSArray<NSObject>> = INSArray::from_vec(vec);
		assert!(array.count() == 4);
	}

	#[test]
	fn test_object_at() {
		let vec: Vec<Id<NSObject>> = Vec::from_fn(4, |_| INSObject::new());
		let array: Id<NSArray<NSObject>> = INSArray::from_vec(vec);
		assert!(array.object_at(0) != array.object_at(3));
	}

	#[test]
	fn test_object_enumerator() {
		let vec: Vec<Id<NSObject>> = Vec::from_fn(4, |_| INSObject::new());
		let array: Id<NSArray<NSObject>> = INSArray::from_vec(vec);

		assert!(array.object_enumerator().count() == 4);
		assert!(array.object_enumerator()
		             .enumerate()
		             .all(|(i, obj)| obj == array.object_at(i)));
	}
}
