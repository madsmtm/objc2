use std::kinds::marker::ContravariantLifetime;
use std::mem;

use runtime::Object;
use {class, Id, IdVector, IntoIdVector, Owned, Ownership};
use super::{INSCopying, INSObject};

pub struct NSRange {
	pub location: uint,
	pub length: uint,
}

pub struct NSEnumerator<'a, T> {
	id: Id<Object>,
	marker: ContravariantLifetime<'a>,
}

impl<'a, T> NSEnumerator<'a, T> {
	pub unsafe fn from_ptr(ptr: *mut Object) -> NSEnumerator<'a, T> {
		NSEnumerator { id: Id::from_ptr(ptr), marker: ContravariantLifetime }
	}
}

impl<'a, T> Iterator<&'a T> for NSEnumerator<'a, T> {
	fn next(&mut self) -> Option<&'a T> {
		unsafe {
			let obj = msg_send![self.id nextObject] as *mut T;
			if obj.is_null() {
				None
			} else {
				Some(&*obj)
			}
		}
	}
}

pub trait INSArray<T: INSObject, O: Ownership> : INSObject {
	fn count(&self) -> uint {
		let result = unsafe {
			msg_send![self count]
		};
		result as uint
	}

	fn object_at(&self, index: uint) -> &T {
		unsafe {
			let result = msg_send![self objectAtIndex:index] as *mut T;
			&*result
		}
	}

	fn object_enumerator(&self) -> NSEnumerator<T> {
		unsafe {
			let result = msg_send![self objectEnumerator];
			NSEnumerator::from_ptr(result)
		}
	}

	unsafe fn from_refs(refs: &[&T]) -> Id<Self> {
		let cls = class::<Self>();
		let obj = msg_send![cls alloc];
		let obj = msg_send![obj initWithObjects:refs.as_ptr() count:refs.len()];
		Id::from_retained_ptr(obj as *mut Self)
	}

	fn from_vec(vec: Vec<Id<T, O>>) -> Id<Self> {
		let refs = vec.as_refs_slice();
		unsafe {
			INSArray::from_refs(refs)
		}
	}

	fn objects_in_range(&self, start: uint, len: uint) -> Vec<&T> {
		let vec: Vec<*mut T> = Vec::from_elem(len, RawPtr::null());
		let range = NSRange { location: start, length: len };
		unsafe {
			msg_send![self getObjects:vec.as_ptr() range:range];
			mem::transmute(vec)
		}
	}

	fn to_vec(&self) -> Vec<&T> {
		self.objects_in_range(0, self.count())
	}

	fn into_vec(array: Id<Self>) -> Vec<Id<T, O>> {
		let vec = array.to_vec();
		unsafe {
			vec.into_id_vec()
		}
	}
}

object_struct!(NSArray<T>)

impl<T: INSObject> INSArray<T, Owned> for NSArray<T> { }

impl<T> INSCopying<NSArray<T>> for NSArray<T> { }

impl<T: INSObject> Collection for NSArray<T> {
	fn len(&self) -> uint {
		self.count()
	}
}

impl<T: INSObject> Index<uint, T> for NSArray<T> {
	fn index(&self, index: &uint) -> &T {
		self.object_at(*index)
	}
}

#[cfg(test)]
mod tests {
	use {Id};
	use foundation::{INSObject, NSObject};
	use super::{INSArray, NSArray};

	fn sample_array(len: uint) -> Id<NSArray<NSObject>> {
		let vec: Vec<Id<NSObject>> = Vec::from_fn(len, |_| INSObject::new());
		INSArray::from_vec(vec)
	}

	#[test]
	fn test_count() {
		let empty_array: Id<NSArray<NSObject>> = INSObject::new();
		assert!(empty_array.count() == 0);

		let array = sample_array(4);
		assert!(array.count() == 4);
	}

	#[test]
	fn test_object_at() {
		let array = sample_array(4);
		assert!(array.object_at(0) != array.object_at(3));
	}

	#[test]
	fn test_object_enumerator() {
		let array = sample_array(4);

		assert!(array.object_enumerator().count() == 4);
		assert!(array.object_enumerator()
		             .enumerate()
		             .all(|(i, obj)| obj == array.object_at(i)));
	}

	#[test]
	fn test_objects_in_range() {
		let array = sample_array(4);

		let middle_objs = array.objects_in_range(1, 2);
		assert!(middle_objs.len() == 2);
		assert!(*middle_objs.get(0) == array.object_at(1));
		assert!(*middle_objs.get(1) == array.object_at(2));

		let empty_objs = array.objects_in_range(1, 0);
		assert!(empty_objs.len() == 0);

		let all_objs = array.objects_in_range(0, 4);
		assert!(all_objs.len() == 4);
	}

	#[test]
	fn test_into_vec() {
		let array = sample_array(4);

		let vec = INSArray::into_vec(array);
		assert!(vec.len() == 4);
	}
}
